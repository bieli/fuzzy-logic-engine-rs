#[derive(Debug, Clone, Copy)]
pub enum Connective {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub antecedent: Vec<Option<String>>, // term names per input variable (None = wildcard)
    pub consequent: Vec<String>,         // term names per output variable
    pub connective: Connective,          // connective type: And, Or
}

impl Rule {
    pub fn new(
        antecedent: Vec<Option<String>>,
        consequent: Vec<String>,
        connective: Connective,
    ) -> Self {
        Self {
            antecedent,
            consequent,
            connective,
        }
    }
}
