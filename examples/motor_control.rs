use fuzzy_logic_engine_rs::{
    fis::{FisType, FuzzyInferenceSystem},
    membership::MembershipKind as M,
    rule::{Connective, Rule},
    term::Term,
    variable::{LinguisticVariable, Range},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut system = FuzzyInferenceSystem::new("Motor Control");

    // Define output variable (motor speed)
    let mut motor = LinguisticVariable::new(
        "Speed",
        Range {
            min: 0.0,
            max: 2000.0,
        },
    );
    motor.add_term(Term::new(
        "fast",
        M::Trapezoid {
            a: 1000.0,
            b: 1200.0,
            c: 1500.0,
            d: 2000.0,
        },
    ));
    motor.add_term(Term::new(
        "slow",
        M::Trapezoid {
            a: 0.0,
            b: 0.0,
            c: 800.0,
            d: 1200.0,
        },
    ));
    system.add_output(motor);

    // Define input variable: Temperature
    let mut temp = LinguisticVariable::new(
        "Temperature",
        Range {
            min: -80.0,
            max: 80.0,
        },
    );
    temp.add_term(Term::new(
        "cold",
        M::Trapezoid {
            a: -80.0,
            b: -80.0,
            c: 0.0,
            d: 20.0,
        },
    ));
    temp.add_term(Term::new(
        "hot",
        M::Trapezoid {
            a: 15.0,
            b: 20.0,
            c: 80.0,
            d: 80.0,
        },
    ));
    system.add_input(temp);

    // Define input variable: Humidity
    let mut hum = LinguisticVariable::new(
        "Humidity",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    hum.add_term(Term::new(
        "dry",
        M::Trapezoid {
            a: 0.0,
            b: 0.0,
            c: 20.0,
            d: 50.0,
        },
    ));
    hum.add_term(Term::new(
        "wet",
        M::Trapezoid {
            a: 40.0,
            b: 70.0,
            c: 100.0,
            d: 100.0,
        },
    ));
    system.add_input(hum);

    // Rules
    // IF temp is hot AND hum is dry THEN motor is fast
    let r1 = Rule::new(
        vec![Some("hot".into()), Some("dry".into())],
        vec!["fast".into()],
        Connective::And,
    );

    // IF temp is cold AND hum is dry THEN motor is very slow (approximate hedge by reusing "slow")
    let r2 = Rule::new(
        vec![Some("cold".into()), Some("dry".into())],
        vec!["slow".into()],
        Connective::And,
    );

    // IF temp is hot AND hum is wet THEN motor is very fast (approximate hedge by reusing "fast")
    let r3 = Rule::new(
        vec![Some("hot".into()), Some("wet".into())],
        vec!["fast".into()],
        Connective::And,
    );

    // IF temp is cold AND hum is wet THEN motor is slow
    let r4 = Rule::new(
        vec![Some("cold".into()), Some("wet".into())],
        vec!["slow".into()],
        Connective::And,
    );

    system.set_rules(vec![r1, r2, r3, r4]);

    // Evaluate
    let temp = 42.0;
    let hum = 45.0;
    let inputs = vec![temp, hum];

    let result = system.compute(FisType::Mamdani, &inputs);
    let out = result.unwrap();
    println!(
        "Inputs: temp={:?}, hum={:?} => motor speed ≈ {:?}",
        temp, hum, out
    );
    assert!(out[0] > 1480.0);
    assert!(out[0] < 1490.0);

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
