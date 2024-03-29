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
        std::fs::read_dir(self)
            .unwrap()
            .filter_map(std::result::Result::ok)
            .filter(|x| x.path().is_dir())
            .map(|x| Mod(x.path()))
            .collect()
    }
}

/// a folder containing YAML and audio files
#[derive(Debug)]
pub struct Mod(std::path::PathBuf);

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
