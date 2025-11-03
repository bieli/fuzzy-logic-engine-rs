use crate::membership::MembershipKind;

#[derive(Debug, Clone)]
pub struct Term {
    pub name: String,
    pub kind: MembershipKind,
}
