use fuzzy_logic_engine_rs::{
    membership::MembershipKind as M,
    rule::{Connective, Rule},
    term::Term,
    variable::{LinguisticVariable, Range},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: add system definition

    // Output: irrigation level (0 = off, 100 = max flow)
    let mut irrigation = LinguisticVariable::new(
        "irrigation",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    // TODO: add output definition

    // Input: soil moisture (% volumetric water content)
    let mut soil = LinguisticVariable::new(
        "soil_moisture",
        Range {
            min: 0.0,
            max: 100.0,
        },
    );
    // TODO: add input definitions

    // Input: rain forecast (mm expected in next 24h)
    let mut rain = LinguisticVariable::new(
        "rain_forecast",
        Range {
            min: 0.0,
            max: 50.0,
        },
    );
    // TODO: add input definitions

    // Input: crop growth stage (0 = germination, 10 = harvest)
    let mut stage = LinguisticVariable::new(
        "growth_stage",
        Range {
            min: 0.0,
            max: 10.0,
        },
    );
    // TODO: add input definitions

    // TODO: Prepare farming dedicated rules:
    // - Dry soil + no rain -> high irrigation (protect seedlings)
    // - Optimal soil + light rain -> low irrigation
    // - Wet soil -> off
    // - Dry soil + late stage -> medium (avoid overwatering near harvest)
    // - Dry soil + early stage -> high (critical water demand)
    // - Heavy rain -> off (save water, avoid runoff)

    // TODO: add example and inference for IoT readings

    Ok(())
}
