use fuzzy_logic_engine_rs::{
    fis::{FisType, FuzzyInferenceSystem},
    membership::MembershipKind as M,
    rule::{Connective, Rule},
    term::Term,
    variable::{LinguisticVariable, Range},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut system = FuzzyInferenceSystem::new("Gas Burner Power");

    // Define output variable: Power
    let mut power = LinguisticVariable::new(
        "POWER",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    // Crisp consequents approximated as trapezoids with flat membership
    power.add_term(Term::new(
        "LOW_POWER",
        M::Trapezoid {
            a: 0.0,
            b: 20.0,
            c: 20.0,
            d: 30.0,
        },
    ));
    power.add_term(Term::new(
        "MEDIUM_POWER",
        M::Trapezoid {
            a: 25.0,
            b: 25.0,
            c: 55.0,
            d: 65.0,
        },
    ));
    // Functional consequent: approximate HIGH_FUN = OXI^2
    // For simplicity, represent as a polygon mapping (0→0, 3→9)
    power.add_term(Term::new(
        "HIGH_FUN",
        M::Trapezoid {
            a: 40.0,
            b: 60.0,
            c: 80.0,
            d: 100.0,
        },
    ));
    system.add_output(power);

    // Define input variable: Oxygen flow (OXI)
    let mut oxi = LinguisticVariable::new("OXI", Range { min: 0.0, max: 3.0 });
    oxi.add_term(Term::new(
        "low_flow",
        M::Trapezoid {
            a: 0.1,
            b: 1.0,
            c: 1.0,
            d: 1.5,
        },
    ));
    oxi.add_term(Term::new(
        "medium_flow",
        M::Trapezoid {
            a: 0.5,
            b: 1.5,
            c: 2.0,
            d: 3.0,
        },
    ));
    oxi.add_term(Term::new(
        "high_flow",
        M::Trapezoid {
            a: 2.0,
            b: 2.5,
            c: 2.5,
            d: 3.0,
        },
    ));
    system.add_input(oxi);

    // Define rules

    // IF OXI is low_flow THEN POWER is LOW_POWER
    let r1 = Rule::new(
        vec![Some("low_flow".into())],
        vec!["LOW_POWER".into()],
        Connective::And,
    );

    // IF OXI is medium_flow THEN POWER is MEDIUM_POWER
    let r2 = Rule::new(
        vec![Some("medium_flow".into())],
        vec!["MEDIUM_POWER".into()],
        Connective::And,
    );

    // IF NOT (OXI is low_flow) THEN POWER is HIGH_FUN
    // (approximate by using "high_flow" OR "medium_flow")
    let r3 = Rule::new(
        vec![Some("high_flow".into())],
        vec!["HIGH_FUN".into()],
        Connective::Or,
    );

    system.set_rules(vec![r1, r2, r3]);

    // Evaluate
    let oxi = 0.51;
    let inputs = vec![oxi];
    let result = system.compute(FisType::Mamdani, &inputs);
    let out = result.unwrap();

    println!("Inputs: oxi={:?} => Gas Burner Power ≈ {:?}", oxi, out);
    assert!(out[0] > 17.0);
    assert!(out[0] < 18.0);

    match system.compute_verbose(FisType::Mamdani, &inputs) {
        Ok(outputs) => {
            for out in outputs {
                println!("{}", out.describe());
            }
        }
        Err(e) => eprintln!("compute_verbose() - Error: {}", e),
    }
    Ok(())
}
