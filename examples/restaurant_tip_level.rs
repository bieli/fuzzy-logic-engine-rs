use fuzzy_logic_engine_rs::{
    fis::{FisType, FuzzyInferenceSystem},
    membership::MembershipKind as M,
    rule::{Connective, Rule},
    term::Term,
    variable::{LinguisticVariable, Range},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut system = FuzzyInferenceSystem::new("Fuzzy logic based TIP system");

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
        "average",
        M::Triangle {
            a: 10.0,
            b: 15.0,
            c: 20.0,
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

    let mut service = LinguisticVariable::new(
        "service",
        Range {
            min: 0.0,
            max: 10.0,
        },
    );
    service.add_term(Term::new(
        "poor",
        M::Gauss {
            sigma: 2.123,
            mu: 0.0,
        },
    ));
    service.add_term(Term::new(
        "normal",
        M::Gauss {
            sigma: 2.123,
            mu: 5.0,
        },
    ));
    service.add_term(Term::new(
        "excellent",
        M::Gauss {
            sigma: 2.123,
            mu: 10.0,
        },
    ));
    system.add_input(service);

    let mut food = LinguisticVariable::new(
        "food",
        Range {
            min: 0.0,
            max: 10.0,
        },
    );
    food.add_term(Term::new(
        "bad",
        M::Trapezoid {
            a: 0.0,
            b: 0.0,
            c: 1.0,
            d: 3.0,
        },
    ));
    food.add_term(Term::new(
        "good",
        M::Trapezoid {
            a: 7.0,
            b: 9.0,
            c: 10.0,
            d: 10.0,
        },
    ));
    system.add_input(food);

    system.set_rules(vec![
        Rule::new(
            vec![Some("poor".into()), Some("bad".into())],
            vec!["small".into()],
            Connective::And,
        ),
        Rule::new(
            vec![Some("normal".into()), None],
            vec!["average".into()],
            Connective::And,
        ),
        Rule::new(
            vec![Some("excellent".into()), Some("good".into())],
            vec!["generous".into()],
            Connective::And,
        ),
    ]);

    let result = system.compute(FisType::Mamdani, &[7.892, 7.41])?;
    println!("{result:?}");
    assert!(result[0] > 17.0);

    Ok(())
}
