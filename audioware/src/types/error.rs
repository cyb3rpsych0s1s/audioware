use red4ext_rs::types::CName;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum BankError {
    #[snafu(display("unknown in banks: {id}"))]
    Unknown { id: CName },
    #[snafu(display("banks contention"))]
    Contention,
}
