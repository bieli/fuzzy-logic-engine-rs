use fuzzy_logic_engine_rs::{
    fis::{FisError, FisType, FuzzyInferenceSystem},
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

#[test]
fn test_compute_verbose_returns_descriptive_output() {
    // Define input variable
    let mut temp = LinguisticVariable::new(
        "Temperature",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    temp.add_term(Term::new(
        "cold",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 50.0,
        },
    ));
    temp.add_term(Term::new(
        "hot",
        M::Triangle {
            a: 50.0,
            b: 100.0,
            c: 100.0,
        },
    ));

    // Define output variable
    let mut fan = LinguisticVariable::new(
        "FanSpeed",
        Range {
            min: 0.0,
            max: 10.0,
        },
    );
    fan.add_term(Term::new(
        "slow",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 5.0,
        },
    ));
    fan.add_term(Term::new(
        "fast",
        M::Triangle {
            a: 5.0,
            b: 10.0,
            c: 10.0,
        },
    ));

    // Build FIS
    let mut fis = FuzzyInferenceSystem::new("Test FIS");
    fis.add_input(temp);
    fis.add_output(fan);

    // Add simple rules: IF cold THEN slow, IF hot THEN fast
    let r1 = Rule::new(
        vec![Some("cold".into())],
        vec!["slow".into()],
        Connective::And,
    );
    let r2 = Rule::new(
        vec![Some("hot".into())],
        vec!["fast".into()],
        Connective::And,
    );
    fis.set_rules(vec![r1, r2]);

    // Evaluate with an input closer to "hot"
    let result = fis.compute_verbose(FisType::Mamdani, &[80.0]).unwrap();

    // We expect one output variable result
    assert_eq!(result.len(), 1);
    let out = &result[0];

    // Check variable name and range
    assert_eq!(out.variable_name, "FanSpeed");
    assert_eq!(out.range.min, 0.0);
    assert_eq!(out.range.max, 10.0);

    // Crisp value should be within the range
    assert!(out.value[0] >= 0.0 && out.value[0] <= 10.0);

    // Best term should be "fast" for input 80
    assert_eq!(out.best_term.as_deref(), Some("fast"));

    // Term kind string should mention "Triangle"
    assert!(out.term_kind.as_ref().unwrap().contains("Triangle"));
}

#[test]
fn test_input_length_error() {
    // Build a minimal FIS with one input variable
    let mut temp = LinguisticVariable::new(
        "Temperature",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    temp.add_term(Term::new(
        "cold",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 50.0,
        },
    ));
    temp.add_term(Term::new(
        "hot",
        M::Triangle {
            a: 50.0,
            b: 100.0,
            c: 100.0,
        },
    ));

    let mut fis = FuzzyInferenceSystem::new("Test FIS");
    fis.add_input(temp);

    // Call compute with the wrong number of inputs (0 instead of 1)
    let result = fis.compute_verbose(FisType::Mamdani, &[]);

    // Assert that we got the expected error
    match result {
        Err(FisError::InputLen { expected, got }) => {
            assert_eq!(expected, 1);
            assert_eq!(got, 0);
        }
        other => panic!("Expected FisError::InputLen, got {:?}", other),
    }
}

#[test]
fn test_term_not_found_error() {
    // Define input variable
    let mut temp = LinguisticVariable::new(
        "Temperature",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    temp.add_term(Term::new(
        "cold",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 50.0,
        },
    ));

    // Define output variable with only one term
    let mut fan = LinguisticVariable::new(
        "FanSpeed",
        Range {
            min: 0.0,
            max: 10.0,
        },
    );
    fan.add_term(Term::new(
        "slow",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 5.0,
        },
    ));

    // Build FIS
    let mut fis = FuzzyInferenceSystem::new("Test FIS");
    fis.add_input(temp);
    fis.add_output(fan);

    // Add a rule that references a non-existent consequent term "fast"
    let bad_rule = Rule::new(
        vec![Some("cold".into())],
        vec!["fast".into()],
        Connective::And,
    );
    fis.set_rules(vec![bad_rule]);

    // Run inference
    let result = fis.compute_verbose(FisType::Mamdani, &[20.0]);

    // Assert that the error is TermNotFound
    match result {
        Err(FisError::TermNotFound(name)) => {
            assert_eq!(name, "fast");
        }
        other => panic!("Expected FisError::TermNotFound, got {:?}", other),
    }
}

#[test]
fn test_wildcard_rule_triggers_none_branch() {
    // Define input variable
    let mut temp = LinguisticVariable::new(
        "Temperature",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    temp.add_term(Term::new(
        "cold",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 50.0,
        },
    ));

    // Define output variable
    let mut fan = LinguisticVariable::new(
        "FanSpeed",
        Range {
            min: 0.0,
            max: 10.0,
        },
    );
    fan.add_term(Term::new(
        "slow",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 5.0,
        },
    ));

    // Build FIS
    let mut fis = FuzzyInferenceSystem::new("Wildcard Test");
    fis.add_input(temp);
    fis.add_output(fan);

    // Rule with a wildcard antecedent: no condition on the input
    let wildcard_rule = Rule::new(vec![None], vec!["slow".into()], Connective::And);
    fis.set_rules(vec![wildcard_rule]);

    // Evaluate with any input (doesn't matter, wildcard should fire)
    let result = fis.compute_verbose(FisType::Mamdani, &[42.0]).unwrap();

    // We expect one output result
    assert_eq!(result.len(), 1);
    let out = &result[0];

    // The best term should be "slow" because the wildcard rule always fires
    assert_eq!(out.best_term.as_deref(), Some("slow"));
}

#[test]
fn test_duplicate_trapezoid_terms_should_error() {
    // Input variable
    let mut temp = LinguisticVariable::new(
        "Temperature",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    temp.add_term(Term::new(
        "medium",
        M::Trapezoid {
            a: 20.0,
            b: 40.0,
            c: 60.0,
            d: 80.0,
        },
    ));

    // Output variable with two identical terms
    let mut fan = LinguisticVariable::new(
        "FanSpeed",
        Range {
            min: 0.0,
            max: 10.0,
        },
    );
    fan.add_term(Term::new(
        "normal",
        M::Trapezoid {
            a: 2.0,
            b: 4.0,
            c: 6.0,
            d: 8.0,
        },
    ));
    let add_term_result = fan.add_term(Term::new(
        "normal",
        M::Trapezoid {
            a: 2.0,
            b: 4.0,
            c: 6.0,
            d: 8.0,
        },
    ));

    match add_term_result {
        Err(FisError::DuplicateTerm(term_name)) => {
            assert_eq!(term_name, "normal");
        }
        other => {
            panic!(
                "Expected error due to duplicate terms, but got outputs: {:?}",
                other
            );
        }
    }
}
