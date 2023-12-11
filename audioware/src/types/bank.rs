use std::{
    borrow::BorrowMut,
    collections::HashSet,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
use serde::Deserialize;

use crate::engine::SoundId;

use super::{
    redmod::{Mod, ModName, REDmod},
    voice::Voices,
};

lazy_static! {
    static ref IDS: Arc<Mutex<HashSet<SoundId>>> = Arc::new(Mutex::new(HashSet::new()));
}

pub(super) fn insert(id: SoundId) -> anyhow::Result<bool> {
    if let Ok(mut guard) = IDS.clone().borrow_mut().try_lock() {
        return Ok(guard.insert(id));
    }
    anyhow::bail!("unable to reach ids");
}

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
        self.voices.voices.retain(|id, voice| {
            if voice.validate_args(&folder).is_ok() {
                if let Ok(inserted) = insert(id.clone()) {
                    if !inserted {
                        red4ext_rs::error!("duplicate bank id {id}");
                    }
                    return inserted;
                } else {
                    red4ext_rs::error!("unable to insert bank id {id}");
                }
            }
            false
        });
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
                let voices = serde_yaml::from_slice::<Voices>(content.as_slice())?;

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
