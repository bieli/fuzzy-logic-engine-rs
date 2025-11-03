use crate::term::Term;

#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Clone)]
pub struct LinguisticVariable {
    pub name: String,
    pub range: Range,
    pub terms: Vec<Term>,
}

impl LinguisticVariable {
    pub fn new(name: impl Into<String>, range: Range) -> Self {
        Self {
            name: name.into(),
            range,
            terms: Vec::new(),
        }
    }

    pub fn add_term(&mut self, term: Term) {
        self.terms.push(term);
    }

    pub fn term(&self, name: &str) -> Option<&Term> {
        self.terms.iter().find(|t| t.name == name)
    }
}
