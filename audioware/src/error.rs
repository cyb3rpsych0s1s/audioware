use audioware_manifest::ConversionError;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Conversion error"))]
    Conversion { source: ConversionError },
    #[snafu(display("Internal error"))]
    Internal { source: InternalError },
}

#[derive(Debug, Snafu)]
pub enum InternalError {
    #[snafu(display("{origin} contention"))]
    Contention { origin: &'static str },
}

impl From<InternalError> for Error {
    fn from(source: InternalError) -> Self {
        Self::Internal { source }
    }
}

impl From<ConversionError> for Error {
    fn from(source: ConversionError) -> Self {
        Self::Conversion { source }
    }
}
