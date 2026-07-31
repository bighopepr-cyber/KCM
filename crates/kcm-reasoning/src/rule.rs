use kcm_core::types::*;
use std::collections::HashMap;

pub type RuleID = u32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RulePattern {
    Triple(Option<SubjectID>, PredicateID, Option<ObjectID>),
    And(Box<RulePattern>, Box<RulePattern>),
    Or(Box<RulePattern>, Box<RulePattern>),
    Not(Box<RulePattern>),
}

impl RulePattern {
    pub fn subject_predicate_object(
        s: Option<SubjectID>,
        p: PredicateID,
        o: Option<ObjectID>,
    ) -> Self {
        RulePattern::Triple(s, p, o)
    }

    pub fn and(left: RulePattern, right: RulePattern) -> Self {
        RulePattern::And(Box::new(left), Box::new(right))
    }

    pub fn or(left: RulePattern, right: RulePattern) -> Self {
        RulePattern::Or(Box::new(left), Box::new(right))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not(pattern: RulePattern) -> Self {
        RulePattern::Not(Box::new(pattern))
    }
}

pub type ConfidenceFormula = Box<dyn Fn(&[f64]) -> f64 + Send + Sync>;

pub struct Rule {
    pub id: RuleID,
    pub name: String,
    pub description: String,
    pub pattern: RulePattern,
    pub consequent_predicate: PredicateID,
    pub confidence_formula: ConfidenceFormula,
    pub enabled: bool,
    pub priority: i32,
}

impl Rule {
    pub fn new(
        id: RuleID,
        name: String,
        pattern: RulePattern,
        consequent_predicate: PredicateID,
        confidence_formula: ConfidenceFormula,
    ) -> Self {
        Rule {
            id,
            name,
            description: String::new(),
            pattern,
            consequent_predicate,
            confidence_formula,
            enabled: true,
            priority: 0,
        }
    }

    pub fn with_description(mut self, desc: String) -> Self {
        self.description = desc;
        self
    }

    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

pub struct RuleRegistry {
    rules: HashMap<RuleID, Rule>,
}

impl RuleRegistry {
    pub fn new() -> Self {
        RuleRegistry {
            rules: HashMap::new(),
        }
    }

    pub fn register(&mut self, rule: Rule) -> Result<(), KcmError> {
        if self.rules.contains_key(&rule.id) {
            return Err(KcmError::Conflict(format!(
                "Rule with ID {} already exists",
                rule.id
            )));
        }
        self.rules.insert(rule.id, rule);
        Ok(())
    }

    pub fn get(&self, id: RuleID) -> Option<&Rule> {
        self.rules.get(&id)
    }

    pub fn all_enabled(&self) -> Vec<&Rule> {
        self.rules.values().filter(|r| r.enabled).collect()
    }

    pub fn all(&self) -> Vec<&Rule> {
        self.rules.values().collect()
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

impl Default for RuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
