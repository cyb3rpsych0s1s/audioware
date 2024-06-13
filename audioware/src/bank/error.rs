use snafu::{ensure, Snafu};

use audioware_manifest::Mod;

use super::Id;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("duplicate folder across 'r6\\audioware' and 'mods' folders, skipping folder in 'r6\\audioware' ({folder})"), visibility(pub(crate)))]
    DuplicateAcrossDepots { folder: String },
    #[snafu(visibility(pub(crate)))]
    Registry { source: self::registry::Error },
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
    #[snafu(display("cannot store data: {id}"), visibility(pub(crate)))]
    CannotStoreData { id: Id, path: String },
    #[snafu(display("cannot store subtitle"), visibility(pub(crate)))]
    CannotStoreSubtitle,
    #[snafu(display("cannot store audio settings"), visibility(pub(crate)))]
    CannotStoreSettings,
    #[snafu(display("cannot store id: {id}"), visibility(pub(crate)))]
    CannotStoreAgnosticId { id: Id },
}

pub mod registry {
    use audioware_sys::interop::locale::Locale;
    use red4ext_rs::types::CName;
    use snafu::Snafu;

    #[derive(Debug, Snafu)]
    pub enum Error {
        #[snafu(display("missing locale: {} for {}", locale, cname.to_string()), visibility(pub(crate)))]
        MissingLocale { cname: CName, locale: Locale },
        #[snafu(display("requires gender: {}", cname.to_string()), visibility(pub(crate)))]
        RequireGender { cname: CName },
        #[snafu(display("not found: {}", cname.to_string()), visibility(pub(crate)))]
        NotFound { cname: CName },
    }
}

impl From<self::registry::Error> for self::Error {
    fn from(source: self::registry::Error) -> Self {
        Self::Registry { source }
    }
}

/// ensure no duplicate mod folder name across depots: `r6\audioware` and `mods`.
#[inline]
pub fn ensure_no_duplicate_accross_depots(
    redmod_exists: bool,
    r#mod: &Mod,
    mods: &[Mod],
) -> Result<(), Error> {
    ensure!(
        !redmod_exists || !mods.iter().any(|x| x.same_folder_name(r#mod.as_ref())),
        DuplicateAcrossDepotsSnafu {
            folder: r#mod.as_ref().display().to_string()
        }
    );
    Ok(())
}
