use fuzzy_logic_engine_rs::{
    membership::MembershipKind as M,
    rule::{Connective, Rule},
    term::Term,
    variable::{LinguisticVariable, Range},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: add system definition

    // Output: speed
    let mut speed = LinguisticVariable::new(
        "speed",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    // TODO: add output definition

    // Input: distance to obstacle
    let mut distance = LinguisticVariable::new(
        "distance",
        Range {
            min: 0.0,
            max: 200.0,
        },
    );
    // TODO: add input definitions

    // Input: battery level
    let mut battery = LinguisticVariable::new(
        "battery",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    // TODO: add input definitions

    // TODO: add rules

    // TODO: add rules execution and show results
    Ok(())
}
