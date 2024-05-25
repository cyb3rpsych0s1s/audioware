use red4ext_rs::types::CName;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    Bank { source: BankError },
    Internal { source: InternalError },
}

#[derive(Debug, Snafu)]
pub enum BankError {
    #[snafu(display("unknown in banks: {id}"))]
    Unknown { id: CName },
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
