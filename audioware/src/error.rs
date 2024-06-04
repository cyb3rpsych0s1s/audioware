use std::sync::{MutexGuard, PoisonError, RwLockReadGuard, RwLockWriteGuard, TryLockError};

use audioware_sys::error::ConversionError;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("uninitialized: {which}"), visibility(pub(crate)))]
    Uninitialized { which: &'static str },
    #[snafu(display("write contention: {which}"), visibility(pub(crate)))]
    WriteContention { which: &'static str },
    #[snafu(display("read contention: {which}"), visibility(pub(crate)))]
    ReadContention { which: &'static str },
    #[snafu(display("contention: {which}"), visibility(pub(crate)))]
    Contention { which: &'static str },
    #[snafu(visibility(pub(crate)))]
    InvalidLocale { source: ConversionError },
    #[snafu(visibility(pub(crate)))]
    CannotSet { which: &'static str },
}

impl<'a, T> From<TryLockError<RwLockWriteGuard<'a, T>>> for Error {
    fn from(_: TryLockError<RwLockWriteGuard<'a, T>>) -> Self {
        Self::WriteContention {
            which: std::any::type_name::<T>(),
        }
    }
}

impl<'a, T> From<TryLockError<RwLockReadGuard<'a, T>>> for Error {
    fn from(_: TryLockError<RwLockReadGuard<'a, T>>) -> Self {
        Self::ReadContention {
            which: std::any::type_name::<T>(),
        }
    }
}

impl<'a, T> From<TryLockError<MutexGuard<'a, T>>> for Error {
    fn from(_: TryLockError<MutexGuard<'a, T>>) -> Self {
        Self::Contention {
            which: std::any::type_name::<T>(),
        }
    }
}

impl<'a, T> From<PoisonError<MutexGuard<'a, T>>> for Error {
    fn from(_: PoisonError<MutexGuard<'a, T>>) -> Self {
        Self::Contention {
            which: std::any::type_name::<T>(),
        }
    }
}
