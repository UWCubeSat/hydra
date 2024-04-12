#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub enum State {
    EmptyState,
    UnsignedState(u64),
    SignedState(i64),
    BoolState(bool),
}

impl From<u64> for State {
    #[inline(always)]
    fn from(value: u64) -> Self {
        Self::UnsignedState(value)
    }
}

impl From<i64> for State {
    #[inline(always)]
    fn from(value: i64) -> Self {
        Self::SignedState(value)
    }
}

impl From<bool> for State {
    #[inline(always)]
    fn from(value: bool) -> Self {
        Self::BoolState(value)
    }
}
