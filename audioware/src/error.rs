use audioware_manifest::ConversionError;
use kira::ResourceLimitReached;
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Conversion error"))]
    Conversion { source: ConversionError },
    #[snafu(display("Internal error"))]
    Internal { source: InternalError },
    #[snafu(display("Engine error"))]
    Engine { source: ResourceLimitReached },
}

#[derive(Debug, Snafu)]
pub enum InternalError {
    #[snafu(display("{origin} contention"))]
    Contention { origin: &'static str },
    #[snafu(display("{origin} cannot be initialized more than once"))]
    Init { origin: &'static str },
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

impl From<ResourceLimitReached> for Error {
    fn from(source: ResourceLimitReached) -> Self {
        Self::Engine { source }
    }
}
