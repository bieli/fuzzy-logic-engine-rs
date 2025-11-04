use crate::{
    math_helpers::{centroid, linspace},
    rule::{Connective, Rule},
    variable::LinguisticVariable,
    variable::Range,
};

#[derive(thiserror::Error, Debug)]
pub enum FisError {
    #[error("input length mismatch: expected {expected}, got {got}")]
    InputLen { expected: usize, got: usize },
    #[error("output count mismatch")]
    OutputMismatch,
    #[error("term not found: {0}")]
    TermNotFound(String),
    #[error("undefined fuzzy inference system type")]
    UndefinedFuzzyInferenceSystemType,
}

#[derive(PartialEq, Debug)]
pub enum FisType {
    Mamdani,
}

pub struct FuzzyInferenceSystem {
    pub name: String,
    pub inputs: Vec<LinguisticVariable>,
    pub outputs: Vec<LinguisticVariable>,
    pub rules: Vec<Rule>,
    // discretization resolution for defuzzification
    pub resolution: usize,
}

impl FuzzyInferenceSystem {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            rules: Vec::new(),
            resolution: 200,
        }
    }

    pub fn add_input(&mut self, var: LinguisticVariable) {
        self.inputs.push(var);
    }

    pub fn add_output(&mut self, var: LinguisticVariable) {
        self.outputs.push(var);
    }

    pub fn set_rules(&mut self, rules: Vec<Rule>) {
        self.rules = rules;
    }

    // Compute precise outputs using selected inference type and centroid defuzzification
    pub fn compute(&self, fis_type: FisType, crisp_inputs: &[f64]) -> Result<Vec<f64>, FisError> {
        if crisp_inputs.len() != self.inputs.len() {
            return Err(FisError::InputLen {
                expected: self.inputs.len(),
                got: crisp_inputs.len(),
            });
        }

        if fis_type == FisType::Mamdani {
            // For each output variable, aggregate membership by max over rule implications
            let mut outputs_crisp = Vec::with_capacity(self.outputs.len());

            for (out_idx, out_var) in self.outputs.iter().enumerate() {
                // Initialize aggregated membership curve across discretized range
                let xs = linspace(out_var.range.min, out_var.range.max, self.resolution);
                let mut agg: Vec<f64> = vec![0.0; xs.len()];

                for rule in &self.rules {
                    // TODO: Rule firing strength from antecedents

                    // TODO: Apply to consequent terms of the current output

                    // TODO: We allow multiple outputs; pick the term that belongs to current out var if present
                }

                let centroid = centroid(&xs, &agg);
                outputs_crisp.push(centroid);
            }

            // TODO: return kind of centroids as vector results?!
            return Ok(outputs_crisp);
        }

        Err(FisError::UndefinedFuzzyInferenceSystemType)
    }
}
