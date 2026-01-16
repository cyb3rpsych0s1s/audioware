use std::sync::{LazyLock, atomic::AtomicUsize};

use crate::ControlId;

static COUNTER: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(0));

pub fn next_control_id() -> ControlId {
    ControlId::new(&COUNTER)
}
