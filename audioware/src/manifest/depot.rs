use crate::{
    manifest::error::{BinaryLocationSnafu, NoFolderSnafu},
    utils::macros::ok_or_return,
};
use rayon::iter::ParallelIterator;
use rayon::{iter::ParallelBridge, slice::ParallelSliceMut};
use snafu::{OptionExt, ResultExt};

use super::error::Error;

fn try_get_folder(folder: impl AsRef<std::path::Path>) -> Result<std::path::PathBuf, Error> {
    let current_folder = std::env::current_exe().context(BinaryLocationSnafu)?;
    Ok(current_folder
        .parent()
        .context(NoFolderSnafu { folder: "plugins" })?
        .parent()
        .context(NoFolderSnafu { folder: "red4ext" })?
        .parent()
        .context(NoFolderSnafu {
            folder: "Cyberpunk 2077",
        })?
        .join(folder))
}

fn is_yaml(file: &std::path::Path) -> bool {
    file.extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(|x| x == "yml" || x == "yaml")
        .unwrap_or(false)
}

/// a folder containing YAML and audio files
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mod(std::path::PathBuf);

impl std::fmt::Display for Mod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl Mod {
    /// check that folder names match, independently of location in file system.
    ///
    /// if folder name cannot be determined, it will return `false`.
    pub fn same_folder_name(&self, rhs: &std::path::Path) -> bool {
        if let (Some(this), Some(that)) = (self.0.file_name(), rhs.file_name()) {
            return this == that;
        }
        false
    }
    pub fn manifests_paths(&self) -> Vec<std::path::PathBuf> {
        let readdir = ok_or_return!(std::fs::read_dir(self.as_ref()), Vec::new());
        let mut paths = readdir
            .into_iter()
            .par_bridge()
            .filter_map(std::result::Result::ok)
            .filter_map(|x| {
                if is_yaml(x.path().as_path()) {
                    Some(x.path())
                } else {
                    None
                }
            })
            .collect::<Vec<std::path::PathBuf>>();
        paths.as_mut_slice().par_sort();
        paths
    }
}

impl AsRef<std::path::Path> for Mod {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_path()
    }
}

#[derive(Debug)]
pub struct REDmod(std::path::PathBuf);

impl REDmod {
    /// try getting REDmod folder, if it exists
    pub fn try_new() -> Result<Self, Error> {
        Ok(Self(try_get_folder("mods")?))
    }
    /// retrieve mods subfolders
    pub fn mods(&self) -> Vec<Mod> {
        let depot = ok_or_return!(Self::try_new(), Vec::new());
        let readdir = ok_or_return!(std::fs::read_dir(depot.as_ref()), Vec::new());
        let mut mods = readdir
            .into_iter()
            .par_bridge()
            .filter_map(std::result::Result::ok)
            .filter(|x| x.path().is_dir())
            .map(|x| Mod(x.path()))
            .collect::<Vec<Mod>>();
        mods.as_mut_slice().par_sort();
        mods
    }
}

impl AsRef<std::path::Path> for REDmod {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_path()
    }
}

#[derive(Debug)]
pub struct R6Audioware(std::path::PathBuf);

impl R6Audioware {
    /// try getting r6\audioware folder, if it exists
    pub fn try_new() -> Result<Self, Error> {
        Ok(Self(try_get_folder(
            std::path::PathBuf::from("r6").join("audioware"),
        )?))
    }
    /// retrieve mods subfolders
    pub fn mods(&self) -> Vec<Mod> {
        let depot = ok_or_return!(Self::try_new(), Vec::new());
        let readdir = ok_or_return!(std::fs::read_dir(depot.as_ref()), Vec::new());
        let mut mods = readdir
            .into_iter()
            .par_bridge()
            .filter_map(std::result::Result::ok)
            .filter(|x| x.path().is_dir())
            .map(|x| Mod(x.path()))
            .collect::<Vec<Mod>>();
        mods.as_mut_slice().par_sort();
        mods
    }
}

impl AsRef<std::path::Path> for R6Audioware {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_path()
    }
}
