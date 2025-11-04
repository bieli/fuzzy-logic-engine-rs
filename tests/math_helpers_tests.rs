use fuzzy_logic_engine_rs::math_helpers::{centroid, linspace};

#[test]
fn test_linspace_generates_correct_number_of_points() {
    let xs = linspace(0.0, 10.0, 6);
    assert_eq!(xs.len(), 6);
    assert!((xs[0] - 0.0).abs() < 1e-6);
    assert!((xs[5] - 10.0).abs() < 1e-6);
}

#[test]
fn test_linspace_is_evenly_spaced() {
    let xs = linspace(0.0, 1.0, 5);
    let diffs: Vec<f64> = xs.windows(2).map(|w| w[1] - w[0]).collect();
    for d in diffs {
        assert!((d - 0.25).abs() < 1e-6);
    }
}

#[test]
fn test_linspace_with_one_point_returns_min() {
    let xs = linspace(5.0, 10.0, 1);
    assert_eq!(xs, vec![5.0]);
}

#[test]
fn test_centroid_of_symmetric_distribution_is_center() {
    let xs = vec![0.0, 1.0, 2.0, 3.0, 4.0];
    let mus = vec![0.0, 1.0, 2.0, 1.0, 0.0]; // symmetric around 2.0
    let c = centroid(&xs, &mus);
    assert!((c - 2.0).abs() < 1e-6);
}

#[test]
fn test_centroid_with_uniform_distribution_is_average() {
    let xs = vec![0.0, 1.0, 2.0, 3.0];
    let mus = vec![1.0, 1.0, 1.0, 1.0]; // flat membership
    let c = centroid(&xs, &mus);
    assert!((c - 1.5).abs() < 1e-6);
}

#[test]
fn test_centroid_with_zero_membership_returns_midpoint() {
    let xs = vec![0.0, 1.0, 2.0, 3.0];
    let mus = vec![0.0, 0.0, 0.0, 0.0]; // no membership
    let c = centroid(&xs, &mus);
    assert_eq!(c, 2.0); // fallback: middle element
}
