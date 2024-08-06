//! Audioware first loads all [`Depot`] from [`REDmod`] and [`R6Audioware`] folders.
//!
//! Each [`Depot`] is expected to contain one or multiple [Manifests](crate::Manifest).

use std::path::{Path, PathBuf};

use crate::error::{BinaryLocationSnafu, NoFolderSnafu};
use rayon::iter::ParallelIterator;
use rayon::{iter::ParallelBridge, slice::ParallelSliceMut};
use snafu::{OptionExt, ResultExt};

use super::error::Error;

#[derive(Debug)]
pub struct REDmod(PathBuf);

impl Depot for REDmod {
    fn try_new() -> Result<Self, Error> {
        Ok(Self(try_get_folder("mods")?))
    }
}

impl AsRef<Path> for REDmod {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

#[derive(Debug)]
pub struct R6Audioware(PathBuf);

impl Depot for R6Audioware {
    /// try getting r6\audioware folder, if it exists
    fn try_new() -> Result<Self, Error> {
        Ok(Self(try_get_folder(PathBuf::from("r6").join("audioware"))?))
    }
}

impl AsRef<Path> for R6Audioware {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

/// a folder containing [`Mod`] folder(s)
pub trait Depot
where
    Self: Sized + AsRef<Path>,
{
    fn try_new() -> Result<Self, Error>;
    /// retrieve mods subfolders
    fn mods(&self) -> Vec<Mod> {
        let depot = match Self::try_new() {
            Ok(x) => x,
            Err(_) => return Vec::new(),
        };
        let readdir = match std::fs::read_dir(depot.as_ref()) {
            Ok(x) => x,
            Err(_) => return Vec::new(),
        };
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

pub fn try_get_folder(folder: impl AsRef<Path>) -> Result<PathBuf, Error> {
    let current_folder = std::env::current_exe().context(BinaryLocationSnafu)?;
    Ok(current_folder
        .parent()
        .context(NoFolderSnafu { folder: "x64" })?
        .parent()
        .context(NoFolderSnafu { folder: "bin" })?
        .parent()
        .context(NoFolderSnafu {
            folder: "Cyberpunk 2077",
        })?
        .join(folder))
}

fn is_yaml(file: &Path) -> bool {
    file.extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(|x| x == "yml" || x == "yaml")
        .unwrap_or(false)
}

/// a folder containing YAML and audio files
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mod(PathBuf);

impl std::fmt::Display for Mod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl Mod {
    /// check that folder names match, independently of location in file system.
    ///
    /// if folder name cannot be determined, it will return `false`.
    pub fn same_folder_name(&self, rhs: &Path) -> bool {
        if let (Some(this), Some(that)) = (self.0.file_name(), rhs.file_name()) {
            return this == that;
        }
        false
    }
    pub fn manifests_paths(&self) -> Vec<PathBuf> {
        let readdir = match std::fs::read_dir(self.as_ref()) {
            Ok(x) => x,
            Err(_) => return Vec::new(),
        };
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
            .collect::<Vec<PathBuf>>();
        paths.as_mut_slice().par_sort();
        paths
    }
}

impl AsRef<Path> for Mod {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

impl Mod {
    /// get mod folder name (file stem)
    pub fn name(&self) -> ModName {
        ModName(self.0.file_stem().unwrap().to_str().unwrap().to_string())
    }
}

/// a mod name
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ModName(String);

impl AsRef<Path> for ModName {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl std::fmt::Display for ModName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
