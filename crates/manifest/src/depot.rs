//! Audioware first loads all [Depot] from [REDmod] and [R6Audioware] folders.
//!
//! Each [Depot] is expected to contain one or multiple [Manifests](crate::Manifest).

use std::path::{Path, PathBuf};

use crate::error::{BinaryLocationSnafu, NoFolderSnafu};
use rayon::iter::ParallelIterator;
use rayon::{iter::ParallelBridge, slice::ParallelSliceMut};
use snafu::{OptionExt, ResultExt};

use super::error::Error;

/// [Depot] for REDmod, `mods`.
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

/// [Depot] for `r6\audioware`.
#[derive(Debug)]
pub struct R6Audioware(PathBuf);

impl Depot for R6Audioware {
    /// Try getting `r6\audioware` folder, if it exists.
    fn try_new() -> Result<Self, Error> {
        Ok(Self(try_get_folder(PathBuf::from("r6").join("audioware"))?))
    }
}

impl AsRef<Path> for R6Audioware {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

/// Folder containing [Mod] folder(s).
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

/// Try to retrieve a specific folder from filesystem.
pub fn try_get_folder(folder: impl AsRef<Path>) -> Result<PathBuf, Error> {
    let current_folder = std::env::current_exe().context(BinaryLocationSnafu)?;
    Ok(game_folder(&current_folder)?.join(folder))
}

/// The folder of the game, from the path of the game executable.
#[cfg(not(target_os = "macos"))]
fn game_folder(current_folder: &Path) -> Result<&Path, Error> {
    current_folder
        .parent()
        .context(NoFolderSnafu { folder: "x64" })?
        .parent()
        .context(NoFolderSnafu { folder: "bin" })?
        .parent()
        .context(NoFolderSnafu {
            folder: "Cyberpunk 2077",
        })
}

/// The folder of the game, from the path of the game executable.
///
/// macOS: the executable is `<game folder>/Cyberpunk2077.app/Contents/MacOS/Cyberpunk2077`, four levels below the folder that holds
/// `r6`, `red4ext` and `archive` (the three `parent()` calls of the Windows layout land on the `.app` bundle). Evidence: the listing of the
/// installed Mac game (`Cyberpunk2077.app`, `r6`, `red4ext`, `archive`, ... side by side) and B1 report 2.2 and 2.4. The bundle is found by
/// its `.app` extension rather than by counting levels.
#[cfg(target_os = "macos")]
fn game_folder(current_folder: &Path) -> Result<&Path, Error> {
    current_folder
        .ancestors()
        .find(|x| x.extension().and_then(std::ffi::OsStr::to_str) == Some("app"))
        .context(NoFolderSnafu {
            folder: "Cyberpunk2077.app",
        })?
        .parent()
        .context(NoFolderSnafu {
            folder: "Cyberpunk 2077",
        })
}

/// A file path as written in a manifest, in a form the host file system understands.
///
/// Manifests are written on Windows, where `\` separates folders. On macOS a `\` is an ordinary character in a file name, so it is
/// replaced by `/` (a Windows-authored `sfx\door.wav` names `sfx/door.wav`). Other platforms return the path unchanged.
pub fn manifest_path(path: &Path) -> std::borrow::Cow<'_, Path> {
    #[cfg(target_os = "macos")]
    if let Some(text) = path.to_str()
        && text.contains('\\')
    {
        return std::borrow::Cow::Owned(PathBuf::from(text.replace('\\', "/")));
    }
    std::borrow::Cow::Borrowed(path)
}

fn is_yaml(file: &Path) -> bool {
    file.extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(|x| x == "yml" || x == "yaml")
        .unwrap_or(false)
}

/// Folder containing YAML manifest and audio files.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mod(PathBuf);

impl std::fmt::Display for Mod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl Mod {
    /// Check that folder names match, independently of location in file system.
    ///
    /// If folder name cannot be determined, it will return `false`.
    pub fn same_folder_name(&self, rhs: &Path) -> bool {
        if let (Some(this), Some(that)) = (self.0.file_name(), rhs.file_name()) {
            return this == that;
        }
        false
    }
    /// Retrieve all manifests paths.
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
    /// Join a file path written in a manifest to the mod folder (see [manifest_path]).
    pub fn join_file(&self, file: impl AsRef<Path>) -> PathBuf {
        self.0.join(manifest_path(file.as_ref()))
    }
}

impl Mod {
    /// Get mod folder name (file stem).
    pub fn name(&self) -> ModName {
        ModName(self.0.file_stem().unwrap().to_str().unwrap().to_string())
    }
}

/// [Mod] name.
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

#[cfg(all(test, target_os = "macos"))]
mod macos_tests {
    use super::*;

    #[test]
    fn game_folder_is_above_the_app_bundle() {
        let exe = Path::new(
            "/Volumes/G/steamapps/common/Cyberpunk 2077/Cyberpunk2077.app/Contents/MacOS/Cyberpunk2077",
        );
        assert_eq!(
            game_folder(exe).unwrap(),
            Path::new("/Volumes/G/steamapps/common/Cyberpunk 2077")
        );
        assert!(game_folder(Path::new("/usr/bin/true")).is_err());
    }

    #[test]
    fn manifest_paths_use_slashes() {
        assert_eq!(
            manifest_path(Path::new(r"sfx\door.wav")).as_ref(),
            Path::new("sfx/door.wav")
        );
        assert_eq!(
            manifest_path(Path::new("sfx/door.wav")).as_ref(),
            Path::new("sfx/door.wav")
        );
    }
}
