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
        source: audioware_core::Error,
    },
    #[snafu(visibility(pub(crate)))]
    BankRegistry {
        source: audioware_bank::error::registry::Error,
    },
    ResourceLimitReached {
        source: kira::ResourceLimitReached,
    },
    CannotCreateAudioManager {
        source: kira::manager::backend::cpal::Error,
    },
}

impl From<audioware_core::Error> for self::Error {
    fn from(source: audioware_core::Error) -> Self {
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
        Self::Internal {
            source: value.into(),
        }
    }
}

impl From<kira::manager::backend::cpal::Error> for Error {
    fn from(source: kira::manager::backend::cpal::Error) -> Self {
        Self::CannotCreateAudioManager { source }
    }
}
