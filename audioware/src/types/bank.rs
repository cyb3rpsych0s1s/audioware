use std::collections::HashMap;

use serde::Deserialize;

use super::{
    redmod::{self, Mod, ModName, REDmod},
    voice::Voices,
};

#[derive(Debug, Clone, Deserialize)]
pub struct Bank {
    #[serde(skip)]
    r#mod: ModName,
    voices: Voices,
}

impl Bank {
    pub fn name(&self) -> &ModName {
        &self.r#mod
    }
    pub fn folder(&self) -> std::path::PathBuf {
        REDmod::try_new().unwrap().as_path().join(&self.r#mod)
    }
    pub fn cleanup(&mut self) {
        use validator::ValidateArgs;
        let folder = self.folder();
        self.voices.voices.retain(|_, voice| voice.validate_args(&folder).is_ok());
    }
}

impl TryFrom<&Mod> for Bank {
    type Error = anyhow::Error;

    fn try_from(value: &Mod) -> Result<Self, Self::Error> {
        // safety: dir already checked
        if let Ok(entry) = std::fs::read_dir(value.as_path()) {
            if let Some(manifest) = entry
                .filter_map(std::result::Result::ok)
                .filter(|x| x.path().is_file())
                .find(|x| is_manifest(&x.path()))
            {
                let content = std::fs::read(manifest.path())?;
                let mut voices = serde_yaml::from_slice::<Voices>(content.as_slice())?;

                return Ok(Self {
                    r#mod: value.name(),
                    voices,
                });
            }
        }
        anyhow::bail!("unable to retrieve mod's bank");
    }
}

/// check if path is valid file named "voices" with YAML extension
fn is_manifest(file: &std::path::Path) -> bool {
    if let Some(name) = file.file_stem().and_then(|x| x.to_str()) {
        if name == "voices.yml" || name == "voices.yaml" {
            return true;
        }
    }
    false
}
