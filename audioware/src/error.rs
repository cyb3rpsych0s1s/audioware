use std::sync::{MutexGuard, RwLockReadGuard, RwLockWriteGuard, TryLockError};

use audioware_sys::error::ConversionError;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("write contention: {which}"), visibility(pub(crate)))]
    WriteContention { which: String },
    #[snafu(display("read contention: {which}"), visibility(pub(crate)))]
    ReadContention { which: String },
    #[snafu(display("contention: {which}"), visibility(pub(crate)))]
    Contention { which: String },
    #[snafu(visibility(pub(crate)))]
    InvalidLocale { source: ConversionError },
}

impl<'a, T> From<TryLockError<RwLockWriteGuard<'a, T>>> for Error {
    fn from(_: TryLockError<RwLockWriteGuard<'a, T>>) -> Self {
        Self::WriteContention {
            which: std::any::type_name::<T>().to_string(),
        }
    }
}

impl<'a, T> From<TryLockError<RwLockReadGuard<'a, T>>> for Error {
    fn from(_: TryLockError<RwLockReadGuard<'a, T>>) -> Self {
        Self::ReadContention {
            which: std::any::type_name::<T>().to_string(),
        }
    }
}

impl<'a, T> From<TryLockError<MutexGuard<'a, T>>> for Error {
    fn from(_: TryLockError<MutexGuard<'a, T>>) -> Self {
        Self::Contention {
            which: std::any::type_name::<T>().to_string(),
        }
    }
}
