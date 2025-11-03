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
