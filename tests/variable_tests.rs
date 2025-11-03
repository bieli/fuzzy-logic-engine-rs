use fuzzy_logic_engine_rs::{
    membership::MembershipKind,
    term::Term,
    variable::{LinguisticVariable, Range},
};

#[test]
fn test_add_and_verify_and_find_term() {
    let mut var = LinguisticVariable::new(
        "temperature",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    var.add_term(Term::new(
        "cold",
        MembershipKind::Triangle {
            a: 0.0,
            b: 0.0,
            c: 50.0,
        },
    ));
    var.add_term(Term::new(
        "hot",
        MembershipKind::Triangle {
            a: 50.0,
            b: 100.0,
            c: 100.0,
        },
    ));

    let cold = var.term("cold").unwrap();
    assert!((cold.degree(25.0) - 0.5).abs() < 1e-6);

    assert!(var.term("unknown").is_none());
}
