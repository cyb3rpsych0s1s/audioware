use audioware_sys::interop::entity::Display;
use kira::{manager::error::PlaySoundError, sound::FromFileError};
use red4ext_rs::types::EntityId;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(
        display("cannot find entity: {}", Display::from(entity_id)),
        visibility(pub(crate))
    )]
    CannotFindEntity { entity_id: EntityId },
    #[snafu(visibility(pub(crate)))]
    CannotPlayStatic { source: PlaySoundError<()> },
    #[snafu(visibility(pub(crate)))]
    CannotPlayStream {
        source: PlaySoundError<FromFileError>,
    },
    #[snafu(visibility(pub(crate)))]
    Internal { source: crate::error::Error },
    #[snafu(visibility(pub(crate)))]
    BankRegistry {
        source: crate::bank::error::registry::Error,
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
