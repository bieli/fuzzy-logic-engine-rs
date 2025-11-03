use fuzzy_logic_engine_rs::{
    membership::MembershipKind as M,
    rule::{Connective, Rule},
    term::Term,
    variable::{LinguisticVariable, Range},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: add system definition

    // Output: savings rate (% of income)
    let mut savings = LinguisticVariable::new(
        "savings",
        Range {
            min: 0.0,
            max: 50.0,
        },
    );
    // TODO: add output definitions

    // Input: income stability (0 = unstable, 10 = very stable)
    let mut stability = LinguisticVariable::new(
        "stability",
        Range {
            min: 0.0,
            max: 10.0,
        },
    );
    // TODO: add input definitions

    // Input: current expenses (% of income)
    let mut expenses = LinguisticVariable::new(
        "expenses",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    // TODO: add input definitions

    // Rules
    // TODO: add rules

    // TODO: process & show results

    Ok(())
}
