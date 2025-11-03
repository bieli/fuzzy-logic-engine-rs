use crate::membership::MembershipKind;

#[derive(Debug, Clone)]
pub struct Term {
    pub name: String,
    pub kind: MembershipKind,
}

impl Term {
    pub fn new(name: impl Into<String>, kind: MembershipKind) -> Self {
        Self { name: name.into(), kind }
    }

    pub fn degree(&self, x: f64) -> f64 {
        self.kind.degree(x)
    }
}
