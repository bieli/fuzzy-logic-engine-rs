use crate::variable::Range;

#[derive(Debug)]
pub struct OutputResult {
    pub variable_name: String,
    pub range: Range,
    pub value: Vec<f64>,
    pub best_term: Option<String>,
    pub term_kind: Option<String>,
}

impl OutputResult {
    pub fn describe(&self) -> String {
        match (&self.best_term, &self.term_kind) {
            (Some(term), Some(kind)) => format!(
                "{} = {:?} in range [{:.1}, {:.1}] -> term '{}' ({})",
                self.variable_name, self.value, self.range.min, self.range.max, term, kind
            ),
            _ => format!(
                "{} = {:?} in range [{:.1}, {:.1}]",
                self.variable_name, self.value, self.range.min, self.range.max
            ),
        }
    }
}
