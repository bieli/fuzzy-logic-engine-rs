use crate::{
    math_helpers::{centroid, linspace},
    output::OutputResult,
    rule::{Connective, Rule},
    variable::LinguisticVariable,
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
        //println!("crisp_inputs.len()={:?}, self.inputs.len()={:?}", crisp_inputs.len(), self.inputs.len());
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
                    // Rule firing strength from antecedents
                    let mut degrees: Vec<f64> = Vec::with_capacity(self.inputs.len());
                    for (i, ant_term_opt) in rule.antecedent.iter().enumerate() {
                        let deg = match ant_term_opt {
                            Some(term_name) => {
                                let var = &self.inputs[i];
                                let term = var
                                    .term(term_name)
                                    .ok_or_else(|| FisError::TermNotFound(term_name.clone()))?;
                                term.degree(crisp_inputs[i])
                            }
                            None => 1.0, // wildcard
                        };
                        degrees.push(deg);
                    }

                    let fire: f64 = match rule.connective {
                        Connective::And => degrees.into_iter().fold(1.0, |a, d| a.min(d)), // min
                        Connective::Or => degrees.into_iter().fold(0.0, |a, d| a.max(d)),  // max
                    };

                    // Apply to consequent terms of the current output
                    // We allow multiple outputs; pick the term that belongs to current out var if present
                    let cons_term_name_opt = rule.consequent.get(out_idx).cloned();
                    if let Some(cons_term_name) = cons_term_name_opt {
                        if let Some(term) = out_var.term(&cons_term_name) {
                            // Aggregate: max between existing agg and clipped term curve
                            for (j, x) in xs.iter().enumerate() {
                                let mu = term.degree(*x);
                                let clipped = fire.min(mu);
                                if clipped > agg[j] {
                                    agg[j] = clipped;
                                }
                            }
                        } else {
                            return Err(FisError::TermNotFound(cons_term_name));
                        }
                    }
                }

                let centroid = centroid(&xs, &agg);
                outputs_crisp.push(centroid);
            }

            // return kind of centroids as vector results?!
            return Ok(outputs_crisp);
        }

        Err(FisError::UndefinedFuzzyInferenceSystemType)
    }

    // Compute outputs and return more descriptive output in OutputResult structure
    pub fn compute_verbose(
        &self,
        fis_type: FisType,
        crisp_inputs: &[f64],
    ) -> Result<Vec<OutputResult>, FisError> {
        let result = self.compute(fis_type, crisp_inputs);

        match result {
            Err(error) => {
                return Err(error);
            }
            Ok(ref other) => {
                let mut results = Vec::new();

                for (out_var, crisp_value) in self.outputs.iter().zip(result.iter()) {
                    // Find the best matching term
                    let mut best_term = None;
                    let mut best_mu = -1.0;
                    let mut term_kind = None;

                    for term in &out_var.terms {
                        let mu = term.membership(&crisp_value.clone());
                        if mu > best_mu {
                            best_mu = mu;
                            best_term = Some(term.name.clone());
                            term_kind = Some(format!("{:?}", term.kind));
                        }
                    }

                    results.push(OutputResult {
                        variable_name: out_var.name.clone(),
                        range: out_var.range.clone(),
                        value: crisp_value.clone(),
                        best_term,
                        term_kind,
                    });
                }

                Ok(results)
            }
        }
    }
}
