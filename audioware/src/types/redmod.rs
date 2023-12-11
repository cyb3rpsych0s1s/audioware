use anyhow::Context;

pub struct REDmod(std::path::PathBuf);

impl REDmod {
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
    pub fn as_path(&self) -> &std::path::Path {
        &self.0.as_path()
    }
    pub fn mods(&self) -> Vec<Mod> {
        std::fs::read_dir(self.as_path())
            .unwrap()
            .filter_map(std::result::Result::ok)
            .filter(|x| x.path().is_dir())
            .map(|x| Mod(x.path()))
            .collect()
    }
}

pub struct Mod(std::path::PathBuf);

impl Mod {
    pub fn as_path(&self) -> &std::path::Path {
        &self.0.as_path()
    }
    pub fn name(&self) -> ModName {
        ModName(self.0.file_stem().unwrap().to_str().unwrap().to_string())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ModName(String);

impl AsRef<std::path::Path> for ModName {
    fn as_ref(&self) -> &std::path::Path {
        &std::path::Path::new(&self.0)
    }
}
