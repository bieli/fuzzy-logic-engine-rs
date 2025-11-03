//TODO: add required includes after library implementations

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut system = FIS::new("Fuzzy logic based TIP system");

    let mut tip = LinguisticVariable::new("tip", Range { min: 0.0, max: 30.0 });
    //TODO: add output definition to fuzzy logic system

    let mut service = LinguisticVariable::new("service", Range { min: 0.0, max: 10.0 });
    //TODO: add input definition to fuzzy logic system

    let mut food = LinguisticVariable::new("food", Range { min: 0.0, max: 10.0 });
    //TODO: add input definition to fuzzy logic system

    //TODO: add rules definition to fuzzy logic system

    let result = system.get_precise_output(&[7.892, 7.41])?;
    println!("{result:?}"); // Expected result: close to [17.4]
    Ok(())
}
