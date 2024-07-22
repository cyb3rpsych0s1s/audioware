use audioware_manifest::ConversionError;
use kira::ResourceLimitReached;
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Conversion error: {source}"))]
    Conversion { source: ConversionError },
    #[snafu(display("Internal error: {source}"))]
    Internal { source: InternalError },
    #[snafu(display("Engine error: {source}"))]
    Engine { source: ResourceLimitReached },
    #[snafu(display("Scene error: {source}"))]
    Scene { source: SceneError },
}

#[derive(Debug, Snafu)]
pub enum InternalError {
    #[snafu(display("{origin} contention"))]
    Contention { origin: &'static str },
    #[snafu(display("{origin} cannot be initialized more than once"))]
    Init { origin: &'static str },
}

#[derive(Debug, Snafu)]
pub enum SceneError {
    #[snafu(display("Only V can be registered as the listener."))]
    InvalidListener,
    #[snafu(display("V cannot be registered as an emitter."))]
    InvalidEmitter,
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

impl From<SceneError> for Error {
    fn from(source: SceneError) -> Self {
        Self::Scene { source }
    }
}
