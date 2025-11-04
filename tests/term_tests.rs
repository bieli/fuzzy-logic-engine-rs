use fuzzy_logic_engine_rs::{membership::MembershipKind as M, term::Term};

#[test]
fn test_term_degree_for_tiangle_membership_kind() {
    let t = Term::new(
        "medium",
        M::Triangle {
            a: 0.0,
            b: 5.0,
            c: 10.0,
        },
    );
    assert_eq!(t.name, "medium");
    assert!((t.degree(5.0) - 1.0).abs() < 1e-6);
}

#[test]
fn test_triangle_membership() {
    let term = Term::new(
        "mid",
        M::Triangle {
            a: 0.0,
            b: 5.0,
            c: 10.0,
        },
    );

    // At the peak
    assert!((term.membership(&vec![5.0]) - 1.0).abs() < 1e-6);

    // At the edges
    assert!((term.membership(&vec![0.0]) - 0.0).abs() < 1e-6);
    assert!((term.membership(&vec![10.0]) - 0.0).abs() < 1e-6);

    // In between
    let mu = term.membership(&vec![2.5]);
    assert!(mu > 0.0 && mu < 1.0);
}

#[test]
fn test_trapezoid_membership() {
    let term = Term::new(
        "plateau",
        M::Trapezoid {
            a: 0.0,
            b: 2.0,
            c: 4.0,
            d: 6.0,
        },
    );

    // Rising edge
    let mu1 = term.membership(&vec![1.0]);
    assert!(mu1 > 0.0 && mu1 < 1.0);

    // Plateau
    assert!((term.membership(&vec![3.0]) - 1.0).abs() < 1e-6);

    // Falling edge
    let mu2 = term.membership(&vec![5.0]);
    assert!(mu2 > 0.0 && mu2 < 1.0);

    // Outside range
    assert_eq!(term.membership(&vec![10.0]), 0.0);
}

#[test]
fn test_gaussian_membership() {
    let term = Term::new(
        "gauss",
        M::Gauss {
            mu: 0.0,
            sigma: 1.0,
        },
    );

    // At mu
    assert!((term.membership(&vec![0.0]) - 1.0).abs() < 1e-6);

    // One sigma away
    let mu_val = term.membership(&vec![1.0]);
    assert!(mu_val < 1.0 && mu_val > 0.0);

    // Far away
    let mu_far = term.membership(&vec![5.0]);
    assert!(mu_far < 0.01);
}

#[test]
fn test_membership_uses_first_element_only() {
    let term = Term::new(
        "tri",
        M::Triangle {
            a: 0.0,
            b: 5.0,
            c: 10.0,
        },
    );

    // membership should only look at the first element of the vector
    let mu1 = term.membership(&vec![5.0, 100.0, -50.0]);
    let mu2 = term.membership(&vec![5.0]);
    assert!((mu1 - mu2).abs() < 1e-6);
}
