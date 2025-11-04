use fuzzy_logic_engine_rs::membership::MembershipKind;

#[test]
fn test_triangle_basic_shape_type_of_membership_kind() {
    let tri = MembershipKind::Triangle {
        a: 0.0,
        b: 5.0,
        c: 10.0,
    };

    // Outside range -> 0
    assert_eq!(tri.degree(-1.0), 0.0);
    assert_eq!(tri.degree(11.0), 0.0);

    // At the peak
    assert_eq!(tri.degree(5.0), 1.0);

    // Rising edge: halfway between a and b
    assert!((tri.degree(2.5) - 0.5).abs() < 1e-6);

    // Falling edge: halfway between b and c
    assert!((tri.degree(7.5) - 0.5).abs() < 1e-6);
}

#[test]
fn test_triangle_edges_type_of_membership_kind() {
    let tri = MembershipKind::Triangle {
        a: 0.0,
        b: 5.0,
        c: 10.0,
    };

    // At exact corners
    assert_eq!(tri.degree(0.0), 0.0);
    assert_eq!(tri.degree(10.0), 0.0);

    // Just inside the edges
    assert!(tri.degree(0.1) > 0.0);
    assert!(tri.degree(9.9) > 0.0);
}

#[test]
fn test_triangle_symmetry_of_membership_kind() {
    let tri = MembershipKind::Triangle {
        a: 0.0,
        b: 5.0,
        c: 10.0,
    };

    // Symmetry around the peak
    let left = tri.degree(3.0);
    let right = tri.degree(7.0);
    assert!((left - right).abs() < 1e-6);
}

#[test]
fn test_trapezoid_basic_shape_type_of_membership_kind() {
    let trap = MembershipKind::Trapezoid {
        a: 0.0,
        b: 2.0,
        c: 4.0,
        d: 6.0,
    };

    // Outside range -> 0
    assert_eq!(trap.degree(-1.0), 0.0);
    assert_eq!(trap.degree(7.0), 0.0);

    // Rising edge
    assert!((trap.degree(1.0) - 0.5).abs() < 1e-6);

    // Plateau
    assert_eq!(trap.degree(3.0), 1.0);

    // Falling edge
    assert!((trap.degree(5.0) - 0.5).abs() < 1e-6);
}

#[test]
fn test_trapezoid_edges_type_of_membership_kind() {
    let trap = MembershipKind::Trapezoid {
        a: 0.0,
        b: 2.0,
        c: 4.0,
        d: 6.0,
    };

    // At exact corners
    assert_eq!(trap.degree(0.0), 0.0);
    assert_eq!(trap.degree(2.0), 1.0);
    assert_eq!(trap.degree(4.0), 1.0);
    assert_eq!(trap.degree(6.0), 0.0);
}

#[test]
fn test_gauss_peak_and_symmetry_type_of_membership_kind() {
    let gauss = MembershipKind::Gauss {
        sigma: 1.0,
        mu: 0.0,
    };

    // Peak at mean
    assert!((gauss.degree(0.0) - 1.0).abs() < 1e-6);

    // Symmetry: degree(-x) == degree(x)
    let left = gauss.degree(-1.0);
    let right = gauss.degree(1.0);
    assert!((left - right).abs() < 1e-6);
}

#[test]
fn test_gauss_falloff_type_of_membership_kind() {
    let gauss = MembershipKind::Gauss {
        sigma: 1.0,
        mu: 0.0,
    };

    // Values further from mean should decrease
    let near = gauss.degree(1.0);
    let far = gauss.degree(3.0);
    assert!(near > far);

    // Very far -> close to 0
    assert!(gauss.degree(10.0) < 1e-10);
}
