use fuzzy_logic_engine_rs::{
    fis::{FisType, FuzzyInferenceSystem},
    membership::MembershipKind as M,
    rule::{Connective, Rule},
    term::Term,
    variable::{LinguisticVariable, Range},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut system = FuzzyInferenceSystem::new("Home finance advisor");

    // Output: savings rate (% of income)
    let mut savings = LinguisticVariable::new(
        "savings",
        Range {
            min: 0.0,
            max: 50.0,
        },
    );
    savings.add_term(Term::new(
        "low",
        M::Triangle {
            a: 0.0,
            b: 5.0,
            c: 15.0,
        },
    ));
    savings.add_term(Term::new(
        "medium",
        M::Triangle {
            a: 10.0,
            b: 20.0,
            c: 30.0,
        },
    ));
    savings.add_term(Term::new(
        "high",
        M::Triangle {
            a: 25.0,
            b: 40.0,
            c: 50.0,
        },
    ));
    system.add_output(savings);

    // Input: income stability (0 = unstable, 10 = very stable)
    let mut stability = LinguisticVariable::new(
        "stability",
        Range {
            min: 0.0,
            max: 10.0,
        },
    );
    stability.add_term(Term::new(
        "unstable",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 4.0,
        },
    ));
    stability.add_term(Term::new(
        "moderate",
        M::Triangle {
            a: 3.0,
            b: 5.0,
            c: 7.0,
        },
    ));
    stability.add_term(Term::new(
        "stable",
        M::Triangle {
            a: 6.0,
            b: 10.0,
            c: 10.0,
        },
    ));
    system.add_input(stability);

    // Input: current expenses (% of income)
    let mut expenses = LinguisticVariable::new(
        "expenses",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    expenses.add_term(Term::new(
        "low",
        M::Triangle {
            a: 0.0,
            b: 20.0,
            c: 40.0,
        },
    ));
    expenses.add_term(Term::new(
        "medium",
        M::Triangle {
            a: 30.0,
            b: 50.0,
            c: 70.0,
        },
    ));
    expenses.add_term(Term::new(
        "high",
        M::Triangle {
            a: 60.0,
            b: 80.0,
            c: 100.0,
        },
    ));
    system.add_input(expenses);

    // Rules
    system.set_rules(vec![
        // If income is stable and expenses are low -> save high
        Rule::new(
            vec![Some("stable".into()), Some("low".into())],
            vec!["high".into()],
            Connective::And,
        ),
        // If income is moderate and expenses are medium -> save medium
        Rule::new(
            vec![Some("moderate".into()), Some("medium".into())],
            vec!["medium".into()],
            Connective::And,
        ),
        // If income is unstable or expenses are high -> save low
        Rule::new(
            vec![Some("unstable".into()), None],
            vec!["low".into()],
            Connective::Or,
        ),
        Rule::new(
            vec![None, Some("high".into())],
            vec!["low".into()],
            Connective::Or,
        ),
    ]);

    // Evaluate
    let stable_income = 8.0; // (8/10)
    let medium_expenses = 45.0; // 45%
    let inputs = vec![stable_income, medium_expenses];

    let result = system.compute(FisType::Mamdani, &inputs);
    let out = result.unwrap();

    println!("Suggested savings rate: {:.2}%", out[0]);

    println!(
        "Inputs: stable_income={:?}, medium_expenses={:?} => Home finance advisor ≈ {:?}",
        stable_income, medium_expenses, out
    );
    assert!(out[0] > 6.0);
    assert!(out[0] < 7.0);

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
