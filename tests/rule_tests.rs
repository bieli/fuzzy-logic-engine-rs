use fuzzy_logic_engine_rs::rule::{Rule, Connective};

#[test]
fn test_create_rule() {
    let rule = Rule::new(
        vec![Some("cold".into()), Some("high".into())],
        vec!["increase".into()],
        Connective::And,
    );

    assert_eq!(rule.antecedent.len(), 2);
    assert_eq!(rule.consequent[0], "increase");
}
