use fuzzy_logic_engine_rs::{term::Term, membership::MembershipKind};

#[test]
fn test_term_degree_for_tiangle_membership_kind() {
    let t = Term::new("medium", MembershipKind::Triangle { a: 0.0, b: 5.0, c: 10.0 });
    assert_eq!(t.name, "medium");
    assert!((t.degree(5.0) - 1.0).abs() < 1e-6);
}
