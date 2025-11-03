use fuzzy_logic_engine_rs::membership::MembershipKind;

#[test]
fn test_triangle_type_of_membership_kind() {
    let unit = MembershipKind::Triangle {
        a: 0.0,
        b: 5.0,
        c: 10.0,
    };
    assert_eq!(unit.degree(0.0), 0.0);
    assert_eq!(unit.degree(5.0), 1.0);
    assert_eq!(unit.degree(10.0), 0.0);
    assert!((unit.degree(2.5) - 0.5).abs() < 1e-6);
}
