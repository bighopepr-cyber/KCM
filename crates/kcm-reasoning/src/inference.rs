use crate::rule::{Rule, RuleID, RulePattern, RuleRegistry};
use kcm_core::types::*;
use kcm_storage::column::Schema;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Derivation {
    pub derived_fact: Fact,
    pub rule_id: RuleID,
    pub confidence_formula_result: f64,
}

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
    timeout_secs: u64,
}

impl InferenceEngine {
    pub fn new() -> Self {
        const DEFAULT_MAX_ITERATIONS: usize = 1000;
        const DEFAULT_CONFIDENCE_THRESHOLD: f64 = 0.3;
        InferenceEngine {
            rule_registry: RuleRegistry::new(),
            max_iterations: DEFAULT_MAX_ITERATIONS,
            confidence_threshold: DEFAULT_CONFIDENCE_THRESHOLD,
            timeout_secs: 60,
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

    pub fn with_timeout_secs(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    pub fn register_rule(&mut self, rule: Rule) -> Result<(), KcmError> {
        self.rule_registry.register(rule)
    }

    pub fn infer_with_stats(
        &self,
        schema: &mut Schema,
    ) -> Result<(Vec<Derivation>, InferenceStats), KcmError> {
        let start = Instant::now();
        let max_duration = std::time::Duration::from_secs(self.timeout_secs);
        let mut all_derived: Vec<Derivation> = Vec::new();
        let mut iterations = 0;
        let mut total_rules = 0;

        let mut derived_set: HashSet<(RuleID, u32, u32)> = HashSet::new();

        for iteration in 0..self.max_iterations {
            iterations = iteration + 1;

            if start.elapsed() > max_duration {
                break;
            }

            let mut new_facts: Vec<Derivation> = Vec::new();
            let mut rules_used = 0;

            let mut enabled_rules: Vec<&Rule> = self.rule_registry.all_enabled();
            enabled_rules.sort_by_key(|b| std::cmp::Reverse(b.priority));

            for rule in enabled_rules {
                if !rule.enabled {
                    continue;
                }

                let matches = self.find_pattern_matches(&rule.pattern, schema)?;

                if !matches.is_empty() {
                    rules_used += 1;
                }

                for (subject, object, confidences) in matches {
                    let confidence = (rule.confidence_formula)(&confidences);

                    if confidence >= self.confidence_threshold {
                        let key = (rule.id, subject.0, object.0);
                        if derived_set.contains(&key) {
                            continue;
                        }

                        let mut fact =
                            Fact::new(subject, rule.consequent_predicate, object, confidence)?;
                        fact.priority = rule.priority.clamp(i8::MIN as i32, i8::MAX as i32) as i8;
                        derived_set.insert(key);
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
                    let Some(s) = schema.subject_col.get(idx) else {
                        continue;
                    };
                    let Some(p) = schema.predicate_col.get(idx) else {
                        continue;
                    };
                    let Some(o) = schema.object_col.get(idx) else {
                        continue;
                    };
                    let Some(c) = schema.confidence_col.get(idx) else {
                        continue;
                    };
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

                Ok(matches)
            }

            RulePattern::And(left, right) => {
                let left_matches = self.find_pattern_matches(left, schema)?;
                let right_matches = self.find_pattern_matches(right, schema)?;

                let mut result = Vec::new();
                for (ls, lo, lc) in &left_matches {
                    for (rs, ro, rc) in &right_matches {
                        let left_obj_right_subj = lo.0 == rs.0;
                        let left_obj_right_obj = lo.0 == ro.0;
                        let left_subj_right_subj = ls.0 == rs.0;

                        if left_obj_right_subj || left_obj_right_obj || left_subj_right_subj {
                            let mut confidences = lc.clone();
                            confidences.extend(rc.iter().copied());
                            result.push((*ls, *ro, confidences));
                        }
                    }
                }

                Ok(result)
            }

            RulePattern::Or(left, right) => {
                let mut left_matches = self.find_pattern_matches(left, schema)?;
                let right_matches = self.find_pattern_matches(right, schema)?;

                let mut seen: HashSet<(u32, u32)> = HashSet::new();
                for m in &left_matches {
                    seen.insert((m.0 .0, m.1 .0));
                }
                for m in right_matches {
                    let key = (m.0 .0, m.1 .0);
                    if seen.insert(key) {
                        left_matches.push(m);
                    }
                }
                Ok(left_matches)
            }

            RulePattern::Not(inner) => {
                let inner_matches = self.find_pattern_matches(inner, schema)?;

                let exclude_set: HashSet<(u32, u8, u32)> = inner_matches
                    .iter()
                    .filter_map(|(s, o, _)| {
                        let idx = (0..schema.len()).find(|&i| {
                            !schema.is_deleted(i)
                                && schema.subject_col.get(i) == Some(s.0)
                                && schema.object_col.get(i) == Some(o.0)
                        })?;
                        let p = schema.predicate_col.get(idx)?;
                        Some((s.0, p, o.0))
                    })
                    .collect();

                let mut result = Vec::new();
                for idx in 0..schema.len() {
                    if schema.is_deleted(idx) {
                        continue;
                    }
                    let Some(s) = schema.subject_col.get(idx) else {
                        continue;
                    };
                    let Some(p) = schema.predicate_col.get(idx) else {
                        continue;
                    };
                    let Some(o) = schema.object_col.get(idx) else {
                        continue;
                    };
                    let Some(c) = schema.confidence_col.get(idx) else {
                        continue;
                    };
                    if !exclude_set.contains(&(s, p, o)) {
                        let s_id = SubjectID(s);
                        let o_id = ObjectID(o);
                        result.push((s_id, o_id, vec![c]));
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
