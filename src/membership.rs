#[derive(Debug, Clone)]
pub enum MembershipKind {
    Triangle { a: f64, b: f64, c: f64 },
    Trapezoid { a: f64, b: f64, c: f64, d: f64 },
    Gauss { sigma: f64, mu: f64 },
}
