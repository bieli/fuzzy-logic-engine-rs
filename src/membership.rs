#[derive(Debug, Clone)]
pub enum MembershipKind {
    Triangle { a: f64, b: f64, c: f64 },
    Trapezoid { a: f64, b: f64, c: f64, d: f64 },
    Gauss { sigma: f64, mu: f64 },
}

impl MembershipKind {
    pub fn degree(&self, x: f64) -> f64 {
        match *self {
            /*
             μ(x)
                 ^
             1.0 |           /\
                 |          /  \
                 |         /    \
             0.5 |        /      \
                 |       /        \
             0.0 |------/----------\--------->
                    a   b          c          x
            */
            MembershipKind::Triangle { a, b, c } => {
                if x <= a || x >= c {
                    0.0
                } else if (x - b).abs() < f64::EPSILON {
                    1.0
                } else if x < b {
                    (x - a) / (b - a)
                } else {
                    (c - x) / (c - b)
                }
            }
            /*
             μ(x)
                 ^
             1.0 |          _________
                 |         /         \
                 |        /           \
             0.5 |       /             \
                 |      /               \
             0.0 |-----/-----------------\--------->
                   a   b                 c    d     x
            */
            MembershipKind::Trapezoid { a, b, c, d } => {
                if x <= a || x >= d {
                    0.0
                } else if x >= b && x <= c {
                    1.0
                } else if x > a && x < b {
                    (x - a) / (b - a)
                } else {
                    (d - x) / (d - c)
                }
            }
            /*
             μ(x)
                 ^
             1.0 |                *
                 |              *   *
                 |            *       *
             0.5 |         *             *
                 |       *                 *
                 |     *                     *
             0.0 |---*-------------------------*--------->
                    μ-σ           μ           μ+σ         x

             Where:
             - σ (sigma) controls the width (spread)
             - μ (mu) is the center of the Gaussian (the peak)
            */
            MembershipKind::Gauss { sigma, mu } => {
                let t = (x - mu) / sigma;
                (-0.5 * t * t).exp()
            }
        }
    }
}
