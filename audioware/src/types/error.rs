use red4ext_rs::types::CName;
use snafu::prelude::*;

use super::id::{AnyId, Id};

#[derive(Debug, Snafu)]
pub enum Error {
    Bank { source: BankError },
    Registry { source: RegistryError },
    Internal { source: InternalError },
}

#[derive(Debug, Snafu)]
pub enum BankError {
    #[snafu(display("unknown in banks: {id}"))]
    Unknown { id: CName },
}

#[derive(Debug, Snafu)]
pub enum RegistryError {
    #[snafu(display("ids contain an AnyId when it should not: {id}"))]
    Corrupted { id: AnyId },
    #[snafu(display("id not found in ids: {id}"))]
    NotFound { id: CName },
}

#[derive(Debug, Snafu)]
pub enum InternalError {
    #[snafu(display("{origin} contention"))]
    Contention { origin: &'static str },
}

impl From<InternalError> for Error {
    fn from(value: InternalError) -> Self {
        Self::Internal { source: value }
    }
}

impl From<BankError> for Error {
    fn from(value: BankError) -> Self {
        Self::Bank { source: value }
    }
}

impl From<RegistryError> for Error {
    fn from(value: RegistryError) -> Self {
        Self::Registry { source: value }
    }
}
