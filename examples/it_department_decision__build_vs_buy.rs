use fuzzy_logic_engine_rs::{
    fis::{FisType, FuzzyInferenceSystem},
    membership::MembershipKind as M,
    rule::{Connective, Rule},
    term::Term,
    variable::{LinguisticVariable, Range},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut system = FuzzyInferenceSystem::new("IT Delivery Decision");

    // Output: decision score (0 = strongly buy, 100 = strongly build)
    let mut decision = LinguisticVariable::new(
        "decision",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    decision.add_term(Term::new(
        "buy",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 40.0,
        },
    ));
    decision.add_term(Term::new(
        "neutral",
        M::Triangle {
            a: 30.0,
            b: 50.0,
            c: 70.0,
        },
    ));
    decision.add_term(Term::new(
        "build",
        M::Triangle {
            a: 60.0,
            b: 100.0,
            c: 100.0,
        },
    ));
    system.add_output(decision);

    // Input: budget (0 = very low, 100 = very high)
    let mut budget = LinguisticVariable::new(
        "budget",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    budget.add_term(Term::new(
        "low",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 40.0,
        },
    ));
    budget.add_term(Term::new(
        "medium",
        M::Triangle {
            a: 30.0,
            b: 50.0,
            c: 70.0,
        },
    ));
    budget.add_term(Term::new(
        "high",
        M::Triangle {
            a: 60.0,
            b: 100.0,
            c: 100.0,
        },
    ));
    system.add_input(budget);

    // Input: internal expertise (0 = none, 10 = very strong)
    let mut expertise = LinguisticVariable::new(
        "expertise",
        Range {
            min: 0.0,
            max: 10.0,
        },
    );
    expertise.add_term(Term::new(
        "low",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 4.0,
        },
    ));
    expertise.add_term(Term::new(
        "medium",
        M::Triangle {
            a: 3.0,
            b: 5.0,
            c: 7.0,
        },
    ));
    expertise.add_term(Term::new(
        "high",
        M::Triangle {
            a: 6.0,
            b: 10.0,
            c: 10.0,
        },
    ));
    system.add_input(expertise);

    // Input: urgency (0 = no rush, 10 = extremely urgent)
    let mut urgency = LinguisticVariable::new(
        "urgency",
        Range {
            min: 0.0,
            max: 10.0,
        },
    );
    urgency.add_term(Term::new(
        "low",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 4.0,
        },
    ));
    urgency.add_term(Term::new(
        "medium",
        M::Triangle {
            a: 3.0,
            b: 5.0,
            c: 7.0,
        },
    ));
    urgency.add_term(Term::new(
        "high",
        M::Triangle {
            a: 6.0,
            b: 10.0,
            c: 10.0,
        },
    ));
    system.add_input(urgency);

    // Input: vendor reliability (0 = poor, 10 = excellent)
    let mut vendor = LinguisticVariable::new(
        "vendor",
        Range {
            min: 0.0,
            max: 10.0,
        },
    );
    vendor.add_term(Term::new(
        "poor",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 4.0,
        },
    ));
    vendor.add_term(Term::new(
        "average",
        M::Triangle {
            a: 3.0,
            b: 5.0,
            c: 7.0,
        },
    ));
    vendor.add_term(Term::new(
        "excellent",
        M::Triangle {
            a: 6.0,
            b: 10.0,
            c: 10.0,
        },
    ));
    system.add_input(vendor);

    // Rules
    system.set_rules(vec![
        // If budget is low and urgency is high → Buy
        Rule::new(
            vec![Some("low".into()), None, Some("high".into()), None],
            vec!["buy".into()],
            Connective::And,
        ),
        // If expertise is high and budget is high → Build
        Rule::new(
            vec![Some("high".into()), Some("high".into()), None, None],
            vec!["build".into()],
            Connective::And,
        ),
        // If vendor is excellent and urgency is high → Buy
        Rule::new(
            vec![None, None, Some("high".into()), Some("excellent".into())],
            vec!["buy".into()],
            Connective::And,
        ),
        // If expertise is low and vendor is poor → Neutral (uncertain)
        Rule::new(
            vec![None, Some("low".into()), None, Some("poor".into())],
            vec!["neutral".into()],
            Connective::And,
        ),
        // If budget is medium and expertise is medium → Neutral
        Rule::new(
            vec![Some("medium".into()), Some("medium".into()), None, None],
            vec!["neutral".into()],
            Connective::And,
        ),
    ]);

    // Evaluate
    let budget = 70.0;
    let expertise = 8.0;
    let urgency = 4.0;
    let vendor = 6.0;
    let inputs = vec![budget, expertise, urgency, vendor];

    let result = system.compute(FisType::Mamdani, &inputs);
    let out = result.unwrap();

    println!("Decision score: {:.2}", out[0]);
    if out[0] < 40.0 {
        println!("Recommendation: BUY from partner");
    } else if out[0] > 60.0 {
        println!("Recommendation: BUILD in-house");
    } else {
        println!("Recommendation: NEUTRAL / further analysis needed");
    }

    println!(
        "Inputs: budget={:?}, expertise={:?}, urgency={:?}, vendor={:?} => IT Delivery Decision ≈ {:?}",
        budget, expertise, urgency, vendor, out
    );
    assert!(out[0] > 80.0);
    assert!(out[0] < 83.0);

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
