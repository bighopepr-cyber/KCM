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

    pub fn register_rule(&mut self, rule: Rule) -> Result<(), KcmError> {
        self.rule_registry.register(rule)
    }

    pub fn infer_forward_chaining(&self, schema: &Schema) -> Result<Vec<(Fact, RuleID)>, KcmError> {
        let mut derived_facts = Vec::new();
        let mut iteration = 0;

        loop {
            iteration += 1;
            if iteration > self.max_iterations {
                break;
            }

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

            derived_facts.extend(new_facts);
        }

        Ok(derived_facts)
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

            RulePattern::Not(_) => Err(KcmError::InvalidArgument(
                "Negation not fully implemented".to_string(),
            )),
        }
    }
}

impl Default for InferenceEngine {
    fn default() -> Self {
        Self::new()
    }
}
