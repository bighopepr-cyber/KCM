use crate::fact::Fact;

pub struct QueryResult {
    facts: Vec<Fact>,
    index: usize,
}

impl QueryResult {
    pub(crate) fn new(facts: Vec<kcm_core::types::Fact>) -> Self {
        let facts = facts.into_iter().map(Fact::from_core).collect();
        QueryResult { facts, index: 0 }
    }

    pub fn len(&self) -> usize {
        self.facts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.facts.is_empty()
    }

    pub fn count(&self) -> usize {
        self.facts.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Fact> {
        self.facts.iter()
    }

    pub fn into_vec(self) -> Vec<Fact> {
        self.facts
    }
}

impl Iterator for QueryResult {
    type Item = Fact;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.facts.len() {
            let fact = self.facts[self.index].clone();
            self.index += 1;
            Some(fact)
        } else {
            None
        }
    }
}

impl ExactSizeIterator for QueryResult {
    fn len(&self) -> usize {
        self.facts.len() - self.index
    }
}
