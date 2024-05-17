use anyhow::Context;

/// REDmod folder
#[derive(Debug)]
pub struct REDmod(std::path::PathBuf);

impl AsRef<std::path::Path> for REDmod {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_path()
    }
}

impl REDmod {
    /// try getting REDmod folder, if it exists
    pub fn try_new() -> anyhow::Result<Self> {
        let current_folder = std::env::current_exe()?;
        Ok(Self(
            current_folder
                .parent()
                .context("plugins folder")?
                .parent()
                .context("red4ext folder")?
                .parent()
                .context("Cyberpunk 2077 folder")?
                .join("mods"),
        ))
    }
    /// retrieve mods subfolders
    pub fn mods(&self) -> Vec<Mod> {
        if let Ok(depot) = Self::try_new() {
            if let Ok(readdir) = std::fs::read_dir(depot) {
                return readdir
                    .filter_map(std::result::Result::ok)
                    .filter(|x| x.path().is_dir())
                    .map(|x| Mod(x.path()))
                    .collect();
            }
        }
        Vec::new()
    }
}

/// temporary r6\audioware folder
#[derive(Debug)]
pub struct R6Audioware(std::path::PathBuf);

impl AsRef<std::path::Path> for R6Audioware {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_path()
    }
}

impl R6Audioware {
    /// try getting r6\audioware folder, if it exists
    pub fn try_new() -> anyhow::Result<Self> {
        let current_folder = std::env::current_exe()?;
        Ok(Self(
            current_folder
                .parent()
                .context("plugins folder")?
                .parent()
                .context("red4ext folder")?
                .parent()
                .context("Cyberpunk 2077 folder")?
                .join("r6")
                .join("audioware"),
        ))
    }
    /// retrieve mods subfolders
    pub fn mods(&self) -> Vec<Mod> {
        if let Ok(depot) = Self::try_new() {
            if let Ok(readdir) = std::fs::read_dir(depot) {
                return readdir
                    .filter_map(std::result::Result::ok)
                    .filter(|x| x.path().is_dir())
                    .map(|x| Mod(x.path()))
                    .collect();
            }
        }
        Vec::new()
    }
}

/// a folder containing YAML and audio files
#[derive(Debug)]
pub struct Mod(std::path::PathBuf);

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
}

impl AsRef<std::path::Path> for Mod {
    fn as_ref(&self) -> &std::path::Path {
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

impl AsRef<std::path::Path> for ModName {
    fn as_ref(&self) -> &std::path::Path {
        std::path::Path::new(&self.0)
    }
}

impl std::fmt::Display for ModName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
