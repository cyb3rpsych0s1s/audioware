use snafu::{ensure, Snafu};

use crate::manifest::depot::Mod;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("duplicate folder across 'r6\\audioware' and 'mods' folders, skipping folder in 'r6\\audioware' ({folder})"), visibility(pub(crate)))]
    DuplicateAcrossDepots { folder: String },
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
