use crate::rule::{Rule, RuleID, RulePattern, RuleRegistry};
use kcm_core::types::*;
use kcm_storage::column::Schema;
use std::time::Instant;

/// Provenance record for a derived fact.
/// Tracks which rule was applied and which source facts led to the derivation.
#[derive(Debug, Clone)]
pub struct Derivation {
    pub derived_fact: Fact,
    pub rule_id: RuleID,
    pub confidence_formula_result: f64,
}

/// Statistics about an inference run.
#[derive(Debug, Clone)]
pub struct InferenceStats {
    pub iterations: usize,
    pub facts_derived: usize,
    pub rules_applied: usize,
    pub duration_ms: u64,
}

pub struct InferenceEngine {
    rule_registry: RuleRegistry,
    max_iterations: usize,
    confidence_threshold: f64,
}

impl InferenceEngine {
    pub fn new() -> Self {
        const DEFAULT_MAX_ITERATIONS: usize = 1000;
        const DEFAULT_CONFIDENCE_THRESHOLD: f64 = 0.3;
        InferenceEngine {
            rule_registry: RuleRegistry::new(),
            max_iterations: DEFAULT_MAX_ITERATIONS,
            confidence_threshold: DEFAULT_CONFIDENCE_THRESHOLD,
        }
    }

    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }

    pub fn with_confidence_threshold(mut self, threshold: f64) -> Self {
        self.confidence_threshold = threshold;
        self
    }

    pub fn register_rule(&mut self, rule: Rule) -> Result<(), KcmError> {
        self.rule_registry.register(rule)
    }

    /// Run inference and return both derivations and statistics.
    pub fn infer_with_stats(
        &self,
        schema: &mut Schema,
    ) -> Result<(Vec<Derivation>, InferenceStats), KcmError> {
        let start = Instant::now();
        let max_duration = std::time::Duration::from_secs(60);
        let mut all_derived: Vec<Derivation> = Vec::new();
        let mut iterations = 0;
        let mut total_rules = 0;

        for iteration in 0..self.max_iterations {
            iterations = iteration + 1;
            if start.elapsed() > max_duration {
                break;
            }

            let mut new_facts: Vec<Derivation> = Vec::new();
            let mut rules_used = 0;

            for rule in self.rule_registry.all_enabled() {
                if !rule.enabled {
                    continue;
                }

                let matches = self.find_pattern_matches(&rule.pattern, schema)?;
                rules_used += 1;

                for (subject, object, confidences) in matches {
                    let confidence = (rule.confidence_formula)(&confidences);

                    if confidence >= self.confidence_threshold {
                        let mut fact =
                            Fact::new(subject, rule.consequent_predicate, object, confidence)?;
                        fact.priority = rule.priority.clamp(i8::MIN as i32, i8::MAX as i32) as i8;
                        new_facts.push(Derivation {
                            derived_fact: fact,
                            rule_id: rule.id,
                            confidence_formula_result: confidence,
                        });
                    }
                }
            }

            total_rules += rules_used;
            if new_facts.is_empty() {
                break;
            }

            for d in &new_facts {
                schema.append_fact(&d.derived_fact)?;
            }
            all_derived.extend(new_facts);
        }

        let duration_ms = start.elapsed().as_millis() as u64;
        let facts_derived = all_derived.len();
        Ok((
            all_derived,
            InferenceStats {
                iterations,
                facts_derived,
                rules_applied: total_rules,
                duration_ms,
            },
        ))
    }

    /// Run forward-chaining inference.
    /// Returns derived facts with their source rule IDs.
    pub fn infer_forward_chaining(
        &self,
        schema: &mut Schema,
    ) -> Result<Vec<(Fact, RuleID)>, KcmError> {
        let (derivations, _stats) = self.infer_with_stats(schema)?;
        Ok(derivations
            .into_iter()
            .map(|d| (d.derived_fact, d.rule_id))
            .collect())
    }

    fn find_pattern_matches(
        &self,
        pattern: &RulePattern,
        schema: &Schema,
    ) -> Result<Vec<(SubjectID, ObjectID, Vec<f64>)>, KcmError> {
        match pattern {
            RulePattern::Triple(subj, pred, obj) => {
                let mut matches = Vec::new();

                for idx in 0..schema.len() {
                    if schema.is_deleted(idx) {
                        continue;
                    }
                    if let Some(s) = schema.subject_col.get(idx) {
                        if let Some(p) = schema.predicate_col.get(idx) {
                            if let Some(o) = schema.object_col.get(idx) {
                                if let Some(c) = schema.confidence_col.get(idx) {
                                    let s_id = SubjectID(s);
                                    let p_id = PredicateID(p);
                                    let o_id = ObjectID(o);

                                    if let Some(subject_filter) = subj {
                                        if *subject_filter != s_id {
                                            continue;
                                        }
                                    }
                                    if *pred != p_id {
                                        continue;
                                    }
                                    if let Some(object_filter) = obj {
                                        if *object_filter != o_id {
                                            continue;
                                        }
                                    }
                                    matches.push((s_id, o_id, vec![c]));
                                }
                            }
                        }
                    }
                }

                Ok(matches)
            }

            RulePattern::And(left, right) => {
                let left_matches = self.find_pattern_matches(left, schema)?;
                let right_matches = self.find_pattern_matches(right, schema)?;

                let mut result = Vec::new();
                for (ls, lo, mut lc) in left_matches {
                    for (rs, ro, rc) in &right_matches {
                        if lo.0 == rs.0 {
                            lc.extend(rc.iter().copied());
                            result.push((ls, *ro, lc.clone()));
                        }
                    }
                }

                Ok(result)
            }

            RulePattern::Or(left, right) => {
                let mut left_matches = self.find_pattern_matches(left, schema)?;
                let right_matches = self.find_pattern_matches(right, schema)?;
                left_matches.extend(right_matches);
                Ok(left_matches)
            }

            RulePattern::Not(inner) => {
                let inner_matches = self.find_pattern_matches(inner, schema)?;
                let inner_pairs: std::collections::HashSet<(u32, u32)> =
                    inner_matches.iter().map(|(s, o, _)| (s.0, o.0)).collect();

                let mut result = Vec::new();
                for idx in 0..schema.len() {
                    if schema.is_deleted(idx) {
                        continue;
                    }
                    if let Some(s) = schema.subject_col.get(idx) {
                        if let Some(_p) = schema.predicate_col.get(idx) {
                            if let Some(o) = schema.object_col.get(idx) {
                                if let Some(c) = schema.confidence_col.get(idx) {
                                    if !inner_pairs.contains(&(s, o)) {
                                        let s_id = SubjectID(s);
                                        let o_id = ObjectID(o);
                                        result.push((s_id, o_id, vec![c]));
                                    }
                                }
                            }
                        }
                    }
                }

                Ok(result)
            }
        }
    }
}

impl Default for InferenceEngine {
    fn default() -> Self {
        Self::new()
    }
}
