#[derive(Debug, Clone)]
pub enum MembershipKind {
    Triangle { a: f64, b: f64, c: f64 },
    Trapezoid { a: f64, b: f64, c: f64, d: f64 },
    Gauss { sigma: f64, mu: f64 },
}

impl MembershipKind {
    pub fn degree(&self, x: f64) -> f64 {
        match *self {
            MembershipKind::Triangle { a, b, c } => {
                if x <= a || x >= c {
                    0.0
                } else if x == b {
                    1.0
                } else if x < b {
                    (x - a) / (b - a)
                } else {
                    (c - x) / (c - b)
                }
            }
            MembershipKind::Trapezoid { .. } => todo!(),
            MembershipKind::Gauss { .. } => todo!(),
        }
    }
}
