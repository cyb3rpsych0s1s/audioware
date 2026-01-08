use portable_atomic::AtomicUsize;
use std::sync::{LazyLock, atomic::Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ControlId(usize);

impl ControlId {
    pub(crate) fn new(generator: &LazyLock<AtomicUsize>) -> Self {
        Self(generator.fetch_add(1, Ordering::Relaxed))
    }
}

impl std::fmt::Display for ControlId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ctid:{}", self.0)
    }
}
