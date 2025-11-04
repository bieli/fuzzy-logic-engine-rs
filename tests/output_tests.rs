use fuzzy_logic_engine_rs::output::OutputResult;
use fuzzy_logic_engine_rs::variable::Range;

#[test]
fn test_outputresult_with_best_term_and_kind() {
    let out = OutputResult {
        variable_name: "Speed".to_string(),
        range: Range {
            min: 0.0,
            max: 2000.0,
        },
        value: vec![1340.0],
        best_term: Some("fast".to_string()),
        term_kind: Some("Trapezoid { a: 1000.0, b: 1200.0, c: 1500.0, d: 2000.0 }".to_string()),
    };

    let desc = out.describe();
    assert!(desc.contains("Speed"));
    assert!(desc.contains("1340"));
    assert!(desc.contains("fast"));
    assert!(desc.contains("Trapezoid"));
}

#[test]
fn test_outputresult_without_best_term() {
    let out = OutputResult {
        variable_name: "Temperature".to_string(),
        range: Range {
            min: -50.0,
            max: 50.0,
        },
        value: vec![0.0],
        best_term: None,
        term_kind: None,
    };

    let desc = out.describe();
    assert!(desc.contains("Temperature"));
    assert!(desc.contains("0.0"));
    assert!(desc.contains("[-50.0, 50.0]"));
    // Should not panic even if no term is set
}

#[test]
fn test_outputresult_debug_trait() {
    let out = OutputResult {
        variable_name: "Humidity".to_string(),
        range: Range {
            min: 0.0,
            max: 100.0,
        },
        value: vec![45.0],
        best_term: Some("wet".to_string()),
        term_kind: Some("Trapezoid { a: 40.0, b: 70.0, c: 100.0, d: 100.0 }".to_string()),
    };

    // Debug formatting should include struct fields
    let dbg_str = format!("{:?}", out);
    assert!(dbg_str.contains("Humidity"));
    assert!(dbg_str.contains("wet"));
}
