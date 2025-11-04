use fuzzy_logic_engine_rs::{
    fis::{FisType, FuzzyInferenceSystem},
    membership::MembershipKind as M,
    rule::{Connective, Rule},
    term::Term,
    variable::{LinguisticVariable, Range},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut system = FuzzyInferenceSystem::new("Smart Office Energy Management (HVAC controller)");

    // Output: HVAC intensity (0 = off, 100 = max)
    let mut hvac = LinguisticVariable::new(
        "hvac",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    hvac.add_term(Term::new(
        "low",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 40.0,
        },
    ));
    hvac.add_term(Term::new(
        "medium",
        M::Triangle {
            a: 30.0,
            b: 50.0,
            c: 70.0,
        },
    ));
    hvac.add_term(Term::new(
        "high",
        M::Triangle {
            a: 60.0,
            b: 100.0,
            c: 100.0,
        },
    ));
    system.add_output(hvac);

    // Input: occupancy (0 = empty, 100 = full)
    let mut occupancy = LinguisticVariable::new(
        "occupancy",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    occupancy.add_term(Term::new(
        "low",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 40.0,
        },
    ));
    occupancy.add_term(Term::new(
        "medium",
        M::Triangle {
            a: 30.0,
            b: 50.0,
            c: 70.0,
        },
    ));
    occupancy.add_term(Term::new(
        "high",
        M::Triangle {
            a: 60.0,
            b: 100.0,
            c: 100.0,
        },
    ));
    system.add_input(occupancy);

    // Input: outside temperature (°C, -10 to 40)
    let mut temperature = LinguisticVariable::new(
        "temperature",
        Range {
            min: -10.0,
            max: 40.0,
        },
    );
    temperature.add_term(Term::new(
        "cold",
        M::Triangle {
            a: -10.0,
            b: -10.0,
            c: 10.0,
        },
    ));
    temperature.add_term(Term::new(
        "mild",
        M::Triangle {
            a: 5.0,
            b: 20.0,
            c: 25.0,
        },
    ));
    temperature.add_term(Term::new(
        "hot",
        M::Triangle {
            a: 20.0,
            b: 40.0,
            c: 40.0,
        },
    ));
    system.add_input(temperature);

    // Input: energy price (0 = very cheap, 100 = very expensive)
    let mut price = LinguisticVariable::new(
        "price",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    price.add_term(Term::new(
        "low",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 40.0,
        },
    ));
    price.add_term(Term::new(
        "medium",
        M::Triangle {
            a: 30.0,
            b: 50.0,
            c: 70.0,
        },
    ));
    price.add_term(Term::new(
        "high",
        M::Triangle {
            a: 60.0,
            b: 100.0,
            c: 100.0,
        },
    ));
    system.add_input(price);

    // Rules
    system.set_rules(vec![
        // If occupancy is high and temperature is hot -> HVAC high
        Rule::new(
            vec![Some("high".into()), Some("hot".into()), None],
            vec!["high".into()],
            Connective::And,
        ),
        // If occupancy is low and price is high -> HVAC low
        Rule::new(
            vec![Some("low".into()), None, Some("high".into())],
            vec!["low".into()],
            Connective::And,
        ),
        // If occupancy is medium and temperature is mild -> HVAC medium
        Rule::new(
            vec![Some("medium".into()), Some("mild".into()), None],
            vec!["medium".into()],
            Connective::And,
        ),
        // If price is low -> HVAC can be generous (medium or high)
        Rule::new(
            vec![None, None, Some("low".into())],
            vec!["high".into()],
            Connective::Or,
        ),
    ]);

    // Example scenario: 70% occupancy, 28°C outside, price = 65
    let result = system.compute(FisType::Mamdani, &[70.0, 28.0, 65.0])?;
    println!("HVAC intensity decision: {:.2}%", result[0]);
    assert!(result[0] > 86.0);

    Ok(())
}
