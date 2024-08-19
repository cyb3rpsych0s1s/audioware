use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Registry error: {source}"), visibility(pub(crate)))]
    Registry { source: self::registry::Error },
    #[snafu(display("Validation error: {source}"), visibility(pub(crate)))]
    Validation { source: self::validation::Error },
    #[snafu(display("Manifest error: {source}"), visibility(pub(crate)))]
    Manifest {
        source: audioware_manifest::error::Error,
    },
}

pub mod registry {
    use audioware_manifest::{SpokenLocale, WrittenLocale};
    use red4ext_rs::types::CName;
    use snafu::Snafu;

    #[derive(Debug, Snafu)]
    pub enum Error {
        #[snafu(display("missing spoken locale: {} for {}", locale, cname.to_string()), visibility(pub(crate)))]
        MissingSpokenLocale { cname: CName, locale: SpokenLocale },
        #[snafu(display("missing written locale: {} for {}", locale, cname.to_string()), visibility(pub(crate)))]
        MissingWrittenLocale { cname: CName, locale: WrittenLocale },
        #[snafu(display("requires gender: {}", cname.to_string()), visibility(pub(crate)))]
        RequireGender { cname: CName },
        #[snafu(display("not found: {}", cname.to_string()), visibility(pub(crate)))]
        NotFound { cname: CName },
    }
}

pub mod validation {
    use snafu::Snafu;

    use crate::{Id, Key};

    #[derive(Debug, Snafu)]
    pub enum Error {
        #[snafu(display("duplicate folder across 'r6\\audioware' and 'mods' folders, skipping folder in 'r6\\audioware' ({folder})"), visibility(pub(crate)))]
        DuplicateAcrossDepots { folder: String },
        #[snafu(display("CName already exists: {cname}"), visibility(pub(crate)))]
        NonUniqueKey { cname: String },
        #[snafu(
            display("CName conflicts with existing id: {cname}"),
            visibility(pub(crate))
        )]
        ConflictingKey { cname: String },
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
        #[snafu(display("invalid audio setting"), visibility(pub(crate)))]
        InvalidAudioSetting {
            which: &'static str,
            why: &'static str,
        },
        #[snafu(display("invalid audio caption"), visibility(pub(crate)))]
        InvalidAudioCaption { which: String, why: String },
        #[snafu(display("cannot store data: {key}"), visibility(pub(crate)))]
        CannotStoreData { key: Key, path: String },
        #[snafu(display("cannot store subtitle"), visibility(pub(crate)))]
        CannotStoreSubtitle,
        #[snafu(display("cannot store audio settings"), visibility(pub(crate)))]
        CannotStoreSettings,
        #[snafu(display("cannot store id: {id}"), visibility(pub(crate)))]
        CannotStoreAgnosticId { id: Id },
        #[snafu(display("IO: {source}"), visibility(pub(crate)))]
        IO { source: std::io::Error },
    }
}

impl From<self::registry::Error> for self::Error {
    fn from(source: self::registry::Error) -> Self {
        Self::Registry { source }
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
