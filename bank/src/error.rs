use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Registry error: {source}"), visibility(pub(crate)))]
    Registry { source: self::registry::Error },
    #[snafu(display("Validation error: {source}"), visibility(pub(crate)))]
    Validation { source: self::validation::Error },
    #[snafu(display("Manifest error: {source}"), visibility(pub(crate)))]
    Manifest { source: audioware_manifest::Error },
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

    use crate::Id;

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
        #[snafu(display("cannot load audio: {path}"), visibility(pub(crate)))]
        InvalidAudio {
            path: String,
            source: kira::sound::FromFileError,
        },
        #[snafu(display("invalid audio setting"), visibility(pub(crate)))]
        InvalidAudioSetting {
            which: &'static str,
            why: &'static str,
        },
        #[snafu(display("invalid audio caption"), visibility(pub(crate)))]
        InvalidAudioCaption { which: String, why: String },
        #[snafu(display("cannot store data: {id}"), visibility(pub(crate)))]
        CannotStoreData { id: Id, path: String },
        #[snafu(display("cannot store subtitle"), visibility(pub(crate)))]
        CannotStoreSubtitle,
        #[snafu(display("cannot store audio settings"), visibility(pub(crate)))]
        CannotStoreSettings,
        #[snafu(display("cannot store id: {id}"), visibility(pub(crate)))]
        CannotStoreAgnosticId { id: Id },
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

impl From<audioware_manifest::Error> for self::Error {
    fn from(source: audioware_manifest::Error) -> Self {
        Self::Manifest { source }
    }
}
