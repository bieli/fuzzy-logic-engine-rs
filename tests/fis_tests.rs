use fuzzy_logic_engine_rs::{
    fis::{FisType, FuzzyInferenceSystem},
    membership::MembershipKind as M,
    rule::{Connective, Rule},
    term::Term,
    variable::{LinguisticVariable, Range},
};

#[test]
fn test_simple_tip_system() {
    let mut system = FuzzyInferenceSystem::new("Tip fuzzy test system");

    // Output: tip
    let mut tip = LinguisticVariable::new(
        "tip",
        Range {
            min: 0.0,
            max: 30.0,
        },
    );
    tip.add_term(Term::new(
        "small",
        M::Triangle {
            a: 0.0,
            b: 5.0,
            c: 10.0,
        },
    ));
    tip.add_term(Term::new(
        "generous",
        M::Triangle {
            a: 20.0,
            b: 25.0,
            c: 30.0,
        },
    ));
    system.add_output(tip);

    // Input: service
    let mut service = LinguisticVariable::new(
        "service",
        Range {
            min: 0.0,
            max: 10.0,
        },
    );
    service.add_term(Term::new(
        "poor",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 5.0,
        },
    ));
    service.add_term(Term::new(
        "excellent",
        M::Triangle {
            a: 5.0,
            b: 10.0,
            c: 10.0,
        },
    ));
    system.add_input(service);

    // Rule: IF service is excellent THEN tip is generous
    system.set_rules(vec![Rule::new(
        vec![Some("excellent".into())],
        vec!["generous".into()],
        Connective::And,
    )]);

    let result = system.compute(FisType::Mamdani, &[9.0]).unwrap();
    assert!(result[0] > 20.0); // should lean toward generous
}
