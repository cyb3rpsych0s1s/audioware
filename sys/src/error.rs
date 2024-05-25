use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum ConversionError {
    #[snafu(display("invalid locale: {value}"))]
    InvalidLocale { value: String },
}
