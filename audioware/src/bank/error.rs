use snafu::{ensure, Snafu};

use crate::manifest::depot::Mod;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("duplicate folder across 'r6\\audioware' and 'mods' folders, skipping folder in 'r6\\audioware' ({folder})"), visibility(pub(crate)))]
    DuplicateAcrossDepots { folder: String },
    #[snafu(visibility(pub(crate)))]
    Registry { source: self::registry::Error },
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
