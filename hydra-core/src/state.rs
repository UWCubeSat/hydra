#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub enum State {
    EmptyState,
    UnsignedState(u64),
    SignedState(i64),
    BoolState(bool),
}
