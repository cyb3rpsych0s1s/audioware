//! Bank errors.

use red4ext_rs::types::{CName, Cruid};
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Registry error: {source}"), visibility(pub(crate)))]
    Registry {
        source: self::registry::Error<CName>,
    },
    #[snafu(display("Registry error: {source}"), visibility(pub(crate)))]
    SceneRegistry {
        source: self::registry::Error<Cruid>,
    },
    #[snafu(display("Validation error: {source}"), visibility(pub(crate)))]
    Validation { source: self::validation::Error },
    #[snafu(display("Manifest error: {source}"), visibility(pub(crate)))]
    Manifest {
        source: audioware_manifest::error::Error,
    },
    #[snafu(display("Error(s): {}", errors.iter().map(|e| format!("{e}")).collect::<Vec<_>>().join("\n  -> ")), visibility(pub(crate)))]
    Multiple { errors: Vec<Error> },
}

pub mod registry {
    use audioware_manifest::{SpokenLocale, WrittenLocale};
    use red4ext_rs::types::{CName, Cruid};
    use snafu::Snafu;

    pub trait ErrorDisplay {
        fn error_display(&self) -> impl std::fmt::Display;
    }

    impl ErrorDisplay for CName {
        fn error_display(&self) -> impl std::fmt::Display {
            self.as_str()
        }
    }

    impl ErrorDisplay for Cruid {
        fn error_display(&self) -> impl std::fmt::Display {
            // WolvenKit and Codeware represents CRUID as u64
            // but RED4ext.SDK as i64
            i64::from(*self) as u64
        }
    }

    #[derive(Debug, Snafu)]
    pub enum Error<K: ErrorDisplay> {
        #[snafu(display("missing spoken locale: {} for {}", locale, key.error_display()), visibility(pub(crate)))]
        MissingSpokenLocale { key: K, locale: SpokenLocale },
        #[snafu(display("missing written locale: {} for {}", locale, key.error_display()), visibility(pub(crate)))]
        MissingWrittenLocale { key: K, locale: WrittenLocale },
        #[snafu(display("requires gender: {}", key.error_display()), visibility(pub(crate)))]
        RequireGender { key: K },
        #[snafu(display("not found: {}", key.error_display()), visibility(pub(crate)))]
        NotFound { key: K },
    }
}

pub mod validation {

    use audioware_manifest::{DialogLine, Locale};
    use snafu::Snafu;

    use crate::{Id, Key, SceneId, SceneKey};

    #[derive(Debug, Snafu)]
    pub enum Error {
        #[snafu(
            display(
                "duplicate folder across 'r6\\audioware' and 'mods' folders, skipping folder in 'r6\\audioware' ({folder})"
            ),
            visibility(pub(crate))
        )]
        DuplicateAcrossDepots { folder: String },
        #[snafu(display("CName already exists: {cname}"), visibility(pub(crate)))]
        NonUniqueKey { cname: String },
        #[snafu(
            display("CName conflicts with existing id: {cname}"),
            visibility(pub(crate))
        )]
        ConflictingKey { cname: String },
        #[snafu(
            display("RUID conflicts with existing id: {cruid} for {locale}"),
            visibility(pub(crate))
        )]
        ConflictingSceneKey { cruid: i64, locale: Locale },
        #[snafu(display("audio outside depot: {path}"), visibility(pub(crate)))]
        AudioOutsideDepot { path: String },
        #[snafu(
            display("cannot load audio: {path} ({source})"),
            visibility(pub(crate))
        )]
        InvalidAudio {
            path: String,
            source: kira::sound::FromFileError,
        },
        #[snafu(
            display("invalid cname: {source}"),
            visibility(pub),
            context(suffix(false))
        )]
        /// An error occured while converting audio ID to CName.
        InvalidAudioID { source: std::ffi::NulError },
        #[snafu(display("invalid audio setting(s) {which}: {}", why.iter().map(|x| format!("{}: {}", x.which, x.why)).collect::<Vec<_>>().join("\n")), visibility(pub(crate)))]
        InvalidAudioSettings {
            which: String,
            why: Vec<audioware_manifest::error::ValidationError>,
        },
        #[snafu(
            display("invalid audio caption: {which} ({why})"),
            visibility(pub(crate))
        )]
        InvalidAudioCaption { which: String, why: String },
        #[snafu(display("cannot store data: {key} ({path})"), visibility(pub(crate)))]
        CannotStoreData { key: Key, path: String },
        #[snafu(
            display("cannot store scene data: {key} ({path})"),
            visibility(pub(crate))
        )]
        CannotStoreSceneData { key: SceneKey, path: String },
        #[snafu(
            display("cannot store subtitle: {key} ({value})"),
            visibility(pub(crate))
        )]
        CannotStoreSubtitle { key: Key, value: DialogLine },
        #[snafu(display("cannot store audio settings"), visibility(pub(crate)))]
        CannotStoreSettings,
        #[snafu(display("cannot store id: {id}"), visibility(pub(crate)))]
        CannotStoreAgnosticId { id: Id },
        #[snafu(display("cannot store scene id: {id}"), visibility(pub(crate)))]
        CannotStoreSceneId { id: SceneId },
        #[snafu(display("IO: {source}"), visibility(pub(crate)))]
        IO { source: std::io::Error },
    }
}

impl From<self::registry::Error<CName>> for self::Error {
    fn from(source: self::registry::Error<CName>) -> Self {
        Self::Registry { source }
    }
}

impl From<self::registry::Error<Cruid>> for self::Error {
    fn from(source: self::registry::Error<Cruid>) -> Self {
        Self::SceneRegistry { source }
    }
}

impl From<self::validation::Error> for self::Error {
    fn from(source: self::validation::Error) -> Self {
        Self::Validation { source }
    }
}

impl From<audioware_manifest::error::Error> for self::Error {
    fn from(source: audioware_manifest::error::Error) -> Self {
        Self::Manifest { source }
    }
}

impl From<std::io::Error> for self::Error {
    fn from(source: std::io::Error) -> Self {
        Self::from(validation::Error::IO { source })
    }
}

impl From<std::ffi::NulError> for self::Error {
    fn from(source: std::ffi::NulError) -> Self {
        Self::from(validation::Error::InvalidAudioID { source })
    }
}
