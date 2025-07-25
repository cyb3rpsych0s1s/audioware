//! Plugin errors.

use audioware_manifest::error::ConversionError;
use kira::{PlaySoundError, ResourceLimitReached, sound::FromFileError};
use red4ext_rs::types::{CName, EntityId};
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Conversion error: {source}"))]
    Conversion { source: ConversionError },
    #[snafu(display("Internal error: {source}"))]
    Internal { source: InternalError },
    #[snafu(display("Bank error: {source}"))]
    Bank { source: audioware_bank::Error },
    #[snafu(display("Engine error: {source}"))]
    Engine { source: EngineError },
    #[snafu(display("Scene error: {source}"))]
    Scene { source: SceneError },
    #[snafu(display("Validation error:\n{}", errors.iter().map(|e| format!("- {e}")).collect::<Vec<_>>().join("\n")))]
    Validation {
        errors: Vec<audioware_manifest::error::ValidationError>,
    },
}

#[derive(Debug, Snafu)]
pub enum InternalError {
    #[snafu(display("{origin} contention"))]
    Contention { origin: &'static str },
    #[snafu(display("{origin} cannot be initialized more than once"))]
    Init { origin: &'static str },
}

#[derive(Debug, Snafu)]
pub enum EngineError {
    #[snafu(display("Thread: {source}"))]
    Thread { source: std::io::Error },
    #[snafu(display("Audio manager error: {origin}"))]
    Manager { origin: &'static str },
    #[snafu(display("Resource limit error: {source}"))]
    Limit { source: ResourceLimitReached },
    #[snafu(display("Play sound error: {source}"))]
    Sound { source: PlaySoundError<()> },
    FromFile {
        source: PlaySoundError<FromFileError>,
    },
}

#[derive(Debug, Snafu)]
#[allow(clippy::enum_variant_names)]
pub enum SceneError {
    #[snafu(display("V cannot be registered as an emitter."))]
    InvalidEmitter,
    #[snafu(display("emitter previously registered for tag {} [{}]", tag_name.as_str(), entity_id))]
    DuplicateEmitter {
        entity_id: EntityId,
        tag_name: CName,
    },
    #[snafu(display("emitter is null [{}]", entity_id))]
    MissingEmitter { entity_id: EntityId },
}

#[derive(Debug, Snafu)]
#[allow(clippy::enum_variant_names)]
pub enum ValidationError {
    #[snafu(display("tag_name cannot be empty or n\"None\"."))]
    InvalidTagName,
    #[snafu(display("entity_id cannot be undefined or V."))]
    InvalidTargetId,
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
        Self::Engine {
            source: EngineError::Limit { source },
        }
    }
}

impl From<SceneError> for Error {
    fn from(source: SceneError) -> Self {
        Self::Scene { source }
    }
}

impl From<PlaySoundError<()>> for Error {
    fn from(source: PlaySoundError<()>) -> Self {
        Self::Engine {
            source: EngineError::Sound { source },
        }
    }
}

impl From<PlaySoundError<FromFileError>> for Error {
    fn from(source: PlaySoundError<FromFileError>) -> Self {
        Self::Engine {
            source: EngineError::FromFile { source },
        }
    }
}

impl From<audioware_bank::Error> for Error {
    fn from(source: audioware_bank::Error) -> Self {
        Self::Bank { source }
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Self::Engine {
            source: EngineError::Thread { source },
        }
    }
}

impl From<Vec<audioware_manifest::error::ValidationError>> for Error {
    fn from(errors: Vec<audioware_manifest::error::ValidationError>) -> Self {
        Self::Validation { errors }
    }
}
