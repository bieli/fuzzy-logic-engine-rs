use fuzzy_logic_engine_rs::{
    fis::{FisType, FuzzyInferenceSystem},
    membership::MembershipKind as M,
    rule::{Connective, Rule},
    term::Term,
    variable::{LinguisticVariable, Range},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut system = FuzzyInferenceSystem::new("Robot behaviour");

    // Output: speed
    let mut speed = LinguisticVariable::new(
        "speed",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    speed.add_term(Term::new(
        "stop",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 20.0,
        },
    ));
    speed.add_term(Term::new(
        "slow",
        M::Triangle {
            a: 10.0,
            b: 30.0,
            c: 50.0,
        },
    ));
    speed.add_term(Term::new(
        "fast",
        M::Triangle {
            a: 50.0,
            b: 75.0,
            c: 100.0,
        },
    ));
    system.add_output(speed);

    // Input: distance to obstacle
    let mut distance = LinguisticVariable::new(
        "distance",
        Range {
            min: 0.0,
            max: 200.0,
        },
    );
    distance.add_term(Term::new(
        "near",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 50.0,
        },
    ));
    distance.add_term(Term::new(
        "medium",
        M::Triangle {
            a: 40.0,
            b: 100.0,
            c: 160.0,
        },
    ));
    distance.add_term(Term::new(
        "far",
        M::Triangle {
            a: 120.0,
            b: 200.0,
            c: 200.0,
        },
    ));
    system.add_input(distance);

    // Input: battery level
    let mut battery = LinguisticVariable::new(
        "battery",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    battery.add_term(Term::new(
        "low",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 40.0,
        },
    ));
    battery.add_term(Term::new(
        "medium",
        M::Triangle {
            a: 30.0,
            b: 50.0,
            c: 70.0,
        },
    ));
    battery.add_term(Term::new(
        "high",
        M::Triangle {
            a: 60.0,
            b: 100.0,
            c: 100.0,
        },
    ));
    system.add_input(battery);

    // Rules
    system.set_rules(vec![
        Rule::new(
            vec![Some("near".into()), None],
            vec!["stop".into()],
            Connective::And,
        ),
        Rule::new(
            vec![Some("medium".into()), Some("low".into())],
            vec!["slow".into()],
            Connective::And,
        ),
        Rule::new(
            vec![Some("far".into()), Some("high".into())],
            vec!["fast".into()],
            Connective::And,
        ),
    ]);

    // Evaluate
    let distance_inp = 150.0;
    let battery_inp = 80.0;
    let inputs = vec![distance_inp, battery_inp];

    let result = system.compute(FisType::Mamdani, &inputs);
    let out = result.unwrap();

    println!("Robot speed decision: {:?}", out[0]);

    println!(
        "Inputs: distance_inp={:?}, battery_inp={:?} => Robot behaviour ≈ {:?}",
        distance_inp, battery_inp, out
    );
    assert!(out[0] > 74.0);
    assert!(out[0] < 75.0);

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
