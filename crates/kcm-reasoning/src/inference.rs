use crate::rule::{Rule, RuleID, RulePattern, RuleRegistry};
use kcm_core::types::*;
use kcm_storage::column::Schema;

pub struct InferenceEngine {
    rule_registry: RuleRegistry,
    max_iterations: usize,
    confidence_threshold: f64,
}

impl InferenceEngine {
    pub fn new() -> Self {
        InferenceEngine {
            rule_registry: RuleRegistry::new(),
            max_iterations: 1000,
            confidence_threshold: 0.3,
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

    pub fn infer_forward_chaining(
        &self,
        schema: &mut Schema,
    ) -> Result<Vec<(Fact, RuleID)>, KcmError> {
        let mut all_derived = Vec::new();

        for _iteration in 0..self.max_iterations {
            let mut new_facts = Vec::new();

            for rule in self.rule_registry.all_enabled() {
                if !rule.enabled {
                    continue;
                }

                let matches = self.find_pattern_matches(&rule.pattern, schema)?;

                for (subject, object, confidences) in matches {
                    let confidence = (rule.confidence_formula)(&confidences);

                    if confidence >= self.confidence_threshold {
                        let mut fact =
                            Fact::new(subject, rule.consequent_predicate, object, confidence)?;
                        fact.priority = rule.priority as i8;
                        new_facts.push((fact, rule.id));
                    }
                }
            }

            if new_facts.is_empty() {
                break;
            }

            for (fact, _rule_id) in &new_facts {
                schema.append_fact(fact)?;
            }

            all_derived.extend(new_facts);
        }

        Ok(all_derived)
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
                let inner_subjects: std::collections::HashSet<u32> =
                    inner_matches.iter().map(|(s, _, _)| s.0).collect();
                let inner_objects: std::collections::HashSet<u32> =
                    inner_matches.iter().map(|(_, o, _)| o.0).collect();

                let mut result = Vec::new();
                for idx in 0..schema.len() {
                    if schema.is_deleted(idx) {
                        continue;
                    }
                    if let Some(s) = schema.subject_col.get(idx) {
                        if let Some(_p) = schema.predicate_col.get(idx) {
                            if let Some(o) = schema.object_col.get(idx) {
                                if let Some(c) = schema.confidence_col.get(idx) {
                                    if !inner_subjects.contains(&s) || !inner_objects.contains(&o) {
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
