use fuzzy_logic_engine_rs::{
    fis::{FisType, FuzzyInferenceSystem},
    membership::MembershipKind as M,
    rule::{Connective, Rule},
    term::Term,
    variable::{LinguisticVariable, Range},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut system = FuzzyInferenceSystem::new("Autonomous steering by fuzzy logic");

    // Output: steering angle (negative = left, positive = right)
    let mut steering = LinguisticVariable::new(
        "steering",
        Range {
            min: -30.0,
            max: 30.0,
        },
    );
    steering.add_term(Term::new(
        "left",
        M::Triangle {
            a: -30.0,
            b: -20.0,
            c: -10.0,
        },
    ));
    steering.add_term(Term::new(
        "straight",
        M::Triangle {
            a: -5.0,
            b: 0.0,
            c: 5.0,
        },
    ));
    steering.add_term(Term::new(
        "right",
        M::Triangle {
            a: 10.0,
            b: 20.0,
            c: 30.0,
        },
    ));
    system.add_output(steering);

    // Input: lane deviation (meters from center)
    let mut deviation = LinguisticVariable::new(
        "deviation",
        Range {
            min: -2.0,
            max: 2.0,
        },
    );
    deviation.add_term(Term::new(
        "left",
        M::Triangle {
            a: -2.0,
            b: -2.0,
            c: -0.5,
        },
    ));
    deviation.add_term(Term::new(
        "center",
        M::Triangle {
            a: -0.5,
            b: 0.0,
            c: 0.5,
        },
    ));
    deviation.add_term(Term::new(
        "right",
        M::Triangle {
            a: 0.5,
            b: 2.0,
            c: 2.0,
        },
    ));
    system.add_input(deviation);

    // Input: road curvature (negative = left curve, positive = right curve)
    let mut curvature = LinguisticVariable::new(
        "curvature",
        Range {
            min: -1.0,
            max: 1.0,
        },
    );
    curvature.add_term(Term::new(
        "left",
        M::Triangle {
            a: -1.0,
            b: -1.0,
            c: -0.3,
        },
    ));
    curvature.add_term(Term::new(
        "straight",
        M::Triangle {
            a: -0.2,
            b: 0.0,
            c: 0.2,
        },
    ));
    curvature.add_term(Term::new(
        "right",
        M::Triangle {
            a: 0.3,
            b: 1.0,
            c: 1.0,
        },
    ));
    system.add_input(curvature);

    // Rules
    system.set_rules(vec![
        Rule::new(
            vec![Some("left".into()), Some("straight".into())],
            vec!["right".into()],
            Connective::And,
        ),
        Rule::new(
            vec![Some("right".into()), Some("straight".into())],
            vec!["left".into()],
            Connective::And,
        ),
        Rule::new(
            vec![Some("center".into()), Some("left".into())],
            vec!["left".into()],
            Connective::And,
        ),
        Rule::new(
            vec![Some("center".into()), Some("right".into())],
            vec!["right".into()],
            Connective::And,
        ),
    ]);

    let result = system.compute(FisType::Mamdani, &[-0.8, 0.0])?;
    println!("Steering decision: {:?}", result);
    assert!(result[0] > 19.0);
    assert!(result[0] < 20.0);

    Ok(())
}
