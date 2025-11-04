use fuzzy_logic_engine_rs::{
    fis::{FisType, FuzzyInferenceSystem},
    membership::MembershipKind as M,
    rule::{Connective, Rule},
    term::Term,
    variable::{LinguisticVariable, Range},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut system = FuzzyInferenceSystem::new("Smart Irrigation Controller");

    // Output: irrigation level (0 = off, 100 = max flow)
    let mut irrigation = LinguisticVariable::new(
        "irrigation",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    irrigation.add_term(Term::new(
        "off",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 10.0,
        },
    ));
    irrigation.add_term(Term::new(
        "low",
        M::Triangle {
            a: 5.0,
            b: 20.0,
            c: 35.0,
        },
    ));
    irrigation.add_term(Term::new(
        "medium",
        M::Triangle {
            a: 30.0,
            b: 55.0,
            c: 70.0,
        },
    ));
    irrigation.add_term(Term::new(
        "high",
        M::Triangle {
            a: 65.0,
            b: 85.0,
            c: 100.0,
        },
    ));
    system.add_output(irrigation);

    // Input: soil moisture (% volumetric water content)
    let mut soil = LinguisticVariable::new(
        "soil_moisture",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    soil.add_term(Term::new(
        "dry",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 30.0,
        },
    ));
    soil.add_term(Term::new(
        "optimal",
        M::Triangle {
            a: 25.0,
            b: 45.0,
            c: 65.0,
        },
    ));
    soil.add_term(Term::new(
        "wet",
        M::Triangle {
            a: 60.0,
            b: 100.0,
            c: 100.0,
        },
    ));
    system.add_input(soil);

    // Input: rain forecast (mm expected in next 24h)
    let mut rain = LinguisticVariable::new(
        "rain_forecast",
        Range {
            min: 0.0,
            max: 50.0,
        },
    );
    rain.add_term(Term::new(
        "none",
        M::Triangle {
            a: 0.0,
            b: 0.0,
            c: 5.0,
        },
    ));
    rain.add_term(Term::new(
        "light",
        M::Triangle {
            a: 3.0,
            b: 10.0,
            c: 20.0,
        },
    ));
    rain.add_term(Term::new(
        "heavy",
        M::Triangle {
            a: 15.0,
            b: 50.0,
            c: 50.0,
        },
    ));
    system.add_input(rain);

    // Input: crop growth stage (0 = germination, 10 = harvest)
    let mut stage = LinguisticVariable::new(
        "growth_stage",
        Range {
            min: 0.0,
            max: 10.0,
        },
    );
    stage.add_term(Term::new(
        "early",
        M::Triangle {
            a: 0.0,
            b: 1.5,
            c: 3.0,
        },
    ));
    stage.add_term(Term::new(
        "mid",
        M::Triangle {
            a: 3.0,
            b: 5.0,
            c: 7.0,
        },
    ));
    stage.add_term(Term::new(
        "late",
        M::Triangle {
            a: 7.0,
            b: 9.5,
            c: 10.0,
        },
    ));
    system.add_input(stage);

    // Rules:
    // - Dry soil + no rain -> high irrigation (protect seedlings)
    // - Optimal soil + light rain -> low irrigation
    // - Wet soil -> off
    // - Dry soil + late stage -> medium (avoid overwatering near harvest)
    // - Dry soil + early stage -> high (critical water demand)
    // - Heavy rain -> off (save water, avoid runoff)
    system.set_rules(vec![
        Rule::new(
            vec![
                Some("dry".into()),
                Some("none".into()),
                Some("early".into()),
            ],
            vec!["high".into()],
            Connective::And,
        ),
        Rule::new(
            vec![Some("dry".into()), Some("none".into()), Some("mid".into())],
            vec!["high".into()],
            Connective::And,
        ),
        Rule::new(
            vec![Some("dry".into()), Some("none".into()), Some("late".into())],
            vec!["medium".into()],
            Connective::And,
        ),
        Rule::new(
            vec![Some("optimal".into()), Some("light".into()), None],
            vec!["low".into()],
            Connective::And,
        ),
        Rule::new(
            vec![Some("wet".into()), None, None],
            vec!["off".into()],
            Connective::And,
        ),
        Rule::new(
            vec![None, Some("heavy".into()), None],
            vec!["off".into()],
            Connective::And,
        ),
        // Mild generosity when energy is cheap or evap is high could be added as extra inputs in future
    ]);

    // Evaluate
    // Example IoT readings:
    let soil_moisture = 22.0; // % (dry)
    let rain_forecast = 2.0; // mm (none/light),
    let stage = 2.0; // (early) = 70.0;
    let inputs = vec![soil_moisture, rain_forecast, stage];

    let result = system.compute(FisType::Mamdani, &inputs);
    let out = result.unwrap();

    println!("Irrigation setpoint: {:.2}%", out[0]);

    println!(
        "Inputs: soil_moisture={:?}, rain_forecast={:?}, stage={:?} => Smart Irrigation Controller ≈ {:?}",
        soil_moisture, rain_forecast, stage, out
    );
    assert!(out[0] > 82.0);
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
