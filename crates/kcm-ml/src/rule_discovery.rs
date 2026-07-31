use kcm_core::types::*;
use kcm_reasoning::rule::RulePattern;
use std::collections::HashMap;

pub struct RuleDiscoveryEngine {
    min_support: f64,
    min_confidence: f64,
}

impl RuleDiscoveryEngine {
    pub fn new(min_support: f64, min_confidence: f64) -> Self {
        RuleDiscoveryEngine {
            min_support,
            min_confidence,
        }
    }

    pub fn discover_patterns(&self, facts: &[Fact]) -> Vec<(PredicateID, PredicateID, f64)> {
        let mut chain_counts: HashMap<(u8, u8), usize> = HashMap::new();
        let mut subject_to_facts: HashMap<u32, Vec<(PredicateID, ObjectID)>> = HashMap::new();

        for fact in facts {
            subject_to_facts
                .entry(fact.subject.0)
                .or_default()
                .push((fact.predicate, fact.object));
        }

        for fact in facts {
            if let Some(following_facts) = subject_to_facts.get(&fact.object.0) {
                for (pred2, _) in following_facts {
                    let key = (fact.predicate.0, pred2.0);
                    *chain_counts.entry(key).or_insert(0) += 1;
                }
            }
        }

        let total = facts.len().max(1) as f64;
        let min_count = (total * self.min_support) as usize;

        chain_counts
            .into_iter()
            .filter(|(_, count)| *count >= min_count)
            .map(|((p1, p2), count)| {
                let confidence = (count as f64 / total).clamp(0.0, 1.0);
                (PredicateID(p1), PredicateID(p2), confidence)
            })
            .collect()
    }

    pub fn patterns_to_rules(
        &self,
        patterns: &[(PredicateID, PredicateID, f64)],
    ) -> Vec<(RulePattern, PredicateID, f64)> {
        patterns
            .iter()
            .filter(|(_, _, conf)| *conf >= self.min_confidence)
            .map(|(pred1, pred2, conf)| {
                let pattern = RulePattern::and(
                    RulePattern::subject_predicate_object(None, *pred1, None),
                    RulePattern::subject_predicate_object(None, *pred2, None),
                );
                (pattern, *pred2, *conf)
            })
            .collect()
    }
}
