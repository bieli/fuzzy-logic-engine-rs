use fuzzy_logic_engine_rs::{
    membership::MembershipKind as M,
    rule::{Connective, Rule},
    term::Term,
    variable::{LinguisticVariable, Range},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: add system definition

    // Output: HVAC intensity (0 = off, 100 = max)
    let mut hvac = LinguisticVariable::new(
        "hvac",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    // TODO: add output definition

    // Input: occupancy (0 = empty, 100 = full)
    let mut occupancy = LinguisticVariable::new(
        "occupancy",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    // TODO: add input definitions

    // Input: outside temperature (°C, -10 to 40)
    let mut temperature = LinguisticVariable::new(
        "temperature",
        Range {
            min: -10.0,
            max: 40.0,
        },
    );
    // TODO: add input definitions

    // Input: energy price (0 = very cheap, 100 = very expensive)
    let mut price = LinguisticVariable::new(
        "price",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    // TODO: add input definitions

    // TODO: add HVAC rules

    // TODO: add sample rules execution and show result

    Ok(())
}
