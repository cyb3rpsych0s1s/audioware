use audioware_sys::error::ConversionError;
use red4ext_rs::types::CName;
use snafu::prelude::*;

use super::id::AnyId;

#[derive(Debug, Snafu)]
pub enum Error {
    Bank { source: BankError },
    Registry { source: RegistryError },
    Engine { source: EngineError },
    Internal { source: InternalError },
}

#[derive(Debug, Snafu)]
pub enum EngineError {
    Tracks {
        source: TracksError,
    },
    Scene {
        source: SceneError,
    },
    #[snafu(display("invalid game state {value}"))]
    InvalidState {
        value: u8,
    },
}

#[derive(Debug, Snafu)]
pub enum BankError {
    #[snafu(display("id not found in banks: {id}"), context(suffix(BankSnafu)))]
    NotFound { id: CName },
    #[snafu(display("uninitialized banks"), context(suffix(BankSnafu)))]
    Uninitialized,
    #[snafu(
        display("unable to read audioware dir: {path}"),
        visibility(pub(crate))
    )]
    UnableToReadDir { path: String },
    #[snafu(
        display("unable to read bank manifest: {path}"),
        visibility(pub(crate))
    )]
    UnableToReadManifest {
        path: String,
        source: std::io::Error,
    },
    #[snafu(
        display("invalid bank manifest: {path} ({kind})"),
        visibility(pub(crate))
    )]
    UnableToDeserialize {
        path: String,
        kind: &'static str,
        source: serde_yaml::Error,
    },
}

#[derive(Debug, Snafu)]
pub enum RegistryError {
    #[snafu(display("ids contain an AnyId when it should not: {id}"))]
    Corrupted { id: AnyId },
    #[snafu(display("id not found in ids: {id}"), context(suffix(RegistrySnafu)))]
    NotFound { id: CName },
}

#[derive(Debug, Snafu)]
pub enum TracksError {
    #[snafu(display("uninitialized tracks"), context(suffix(TracksSnafu)))]
    Uninitialized,
    #[snafu(
        display("error setting audio engine tracks"),
        context(suffix(TracksSnafu))
    )]
    Set,
}

#[derive(Debug, Snafu)]
pub enum SceneError {
    #[snafu(display("uninitialized scene"), context(suffix(SceneSnafu)))]
    Uninitialized,
    #[snafu(
        display("error setting audio engine spatial scene"),
        context(suffix(SceneSnafu))
    )]
    Set,
}

#[derive(Debug, Snafu)]
pub enum InternalError {
    #[snafu(display("{origin} contention"))]
    Contention {
        origin: &'static str,
    },
    Conversion {
        origin: ConversionError,
    },
    #[snafu(display("missing {at}"), visibility(pub(crate)))]
    FileSystem {
        at: &'static str,
    },
    #[snafu(display("audio engine resource limit reached"))]
    ResourceLimitReached {
        origin: kira::ResourceLimitReached,
    },
}

impl From<InternalError> for Error {
    fn from(source: InternalError) -> Self {
        Self::Internal { source }
    }
}

impl From<BankError> for Error {
    fn from(source: BankError) -> Self {
        Self::Bank { source }
    }
}

impl From<RegistryError> for Error {
    fn from(source: RegistryError) -> Self {
        Self::Registry { source }
    }
}

impl From<EngineError> for Error {
    fn from(source: EngineError) -> Self {
        Self::Engine { source }
    }
}

impl From<TracksError> for Error {
    fn from(source: TracksError) -> Self {
        Self::Engine {
            source: EngineError::Tracks { source },
        }
    }
}

impl From<SceneError> for Error {
    fn from(source: SceneError) -> Self {
        Self::Engine {
            source: EngineError::Scene { source },
        }
    }
}

impl From<kira::ResourceLimitReached> for Error {
    fn from(origin: kira::ResourceLimitReached) -> Self {
        Self::Internal {
            source: InternalError::ResourceLimitReached { origin },
        }
    }
}
