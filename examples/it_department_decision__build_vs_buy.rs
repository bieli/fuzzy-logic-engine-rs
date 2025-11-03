use fuzzy_logic_engine_rs::{
    membership::MembershipKind as M,
    term::Term,
    variable::{LinguisticVariable, Range},
    rule::{Rule, Connective},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: add system definition

    // Output: decision score (0 = strongly buy, 100 = strongly build)
    let mut decision = LinguisticVariable::new("decision", Range { min: 0.0, max: 100.0 });
    // TODO: add output definition

    // Input: budget (0 = very low, 100 = very high)
    let mut budget = LinguisticVariable::new("budget", Range { min: 0.0, max: 100.0 });
    // TODO: add input definitions

    // Input: internal expertise (0 = none, 10 = very strong)
    let mut expertise = LinguisticVariable::new("expertise", Range { min: 0.0, max: 10.0 });
    // TODO: add input definitions

    // Input: urgency (0 = no rush, 10 = extremely urgent)
    let mut urgency = LinguisticVariable::new("urgency", Range { min: 0.0, max: 10.0 });
    // TODO: add input definitions

    // Input: vendor reliability (0 = poor, 10 = excellent)
    let mut vendor = LinguisticVariable::new("vendor", Range { min: 0.0, max: 10.0 });
    // TODO: add input definitions

    // TODO: set business rule

    // TODO: add recommendations on STDOUT output as result

    Ok(())
}
