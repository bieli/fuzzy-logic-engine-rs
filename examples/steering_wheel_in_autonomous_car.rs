use fuzzy_logic_engine_rs::{
    membership::MembershipKind as M,
    rule::{Connective, Rule},
    term::Term,
    variable::{LinguisticVariable, Range},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: add system definition

    // Output: steering angle (negative = left, positive = right)
    let mut steering = LinguisticVariable::new(
        "steering",
        Range {
            min: -30.0,
            max: 30.0,
        },
    );
    // TODO: add output definitions

    // Input: lane deviation (meters from center)
    let mut deviation = LinguisticVariable::new(
        "deviation",
        Range {
            min: -2.0,
            max: 2.0,
        },
    );
    // TODO: add input definitions

    // Input: road curvature (negative = left curve, positive = right curve)
    let mut curvature = LinguisticVariable::new(
        "curvature",
        Range {
            min: -1.0,
            max: 1.0,
        },
    );
    // TODO: add input definitions

    // TODO: add rules

    // TODO: add applying rules and show results
    Ok(())
}
