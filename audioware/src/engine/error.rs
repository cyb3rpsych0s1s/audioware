use std::sync::{MutexGuard, PoisonError};

use audioware_sys::interop::entity::Display;
use kira::{manager::error::PlaySoundError, sound::FromFileError, ResourceLimitReached};
use red4ext_rs::types::EntityId;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(
        display("cannot find entity: {}", Display::from(entity_id)),
        visibility(pub(crate))
    )]
    CannotFindEntity {
        entity_id: EntityId,
    },
    #[snafu(
        display("invalid modulator value: {reason} ({value})"),
        visibility(pub(crate))
    )]
    InvalidModulatorValue {
        value: f32,
        reason: &'static str,
    },
    #[snafu(visibility(pub(crate)))]
    CannotPlayStatic {
        source: PlaySoundError<()>,
    },
    #[snafu(visibility(pub(crate)))]
    CannotPlayStream {
        source: PlaySoundError<FromFileError>,
    },
    #[snafu(visibility(pub(crate)))]
    Internal {
        source: crate::error::Error,
    },
    #[snafu(visibility(pub(crate)))]
    BankRegistry {
        source: audioware_bank::error::registry::Error,
    },
    ResourceLimitReached {
        source: kira::ResourceLimitReached,
    },
}

impl From<crate::error::Error> for self::Error {
    fn from(source: crate::error::Error) -> Self {
        Self::Internal { source }
    }
}

impl From<PlaySoundError<()>> for Error {
    fn from(source: PlaySoundError<()>) -> Self {
        Self::CannotPlayStatic { source }
    }
}

impl From<PlaySoundError<FromFileError>> for Error {
    fn from(source: PlaySoundError<FromFileError>) -> Self {
        Self::CannotPlayStream { source }
    }
}

impl From<ResourceLimitReached> for Error {
    fn from(source: ResourceLimitReached) -> Self {
        Self::ResourceLimitReached { source }
    }
}

impl<'a, T> From<PoisonError<MutexGuard<'a, T>>> for Error {
    fn from(value: PoisonError<MutexGuard<'a, T>>) -> Self {
        Error::Internal {
            source: value.into(),
        }
    }
}
