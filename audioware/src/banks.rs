use anyhow::Context;
use audioware_types::event::{Event, SoundPlayEvent};
use lazy_static::lazy_static;
use red4ext_rs::types::CName;
use red4ext_rs::types::Ref;
use semver::VersionReq;
use serde::Deserialize;

use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::audio::{Sound, SoundId};
use crate::interop::AudioEvent;

lazy_static! {
    pub static ref BANKS: Arc<Mutex<Banks>> = Arc::new(Mutex::new(Banks::default()));
}

struct REDmod(std::path::PathBuf);
impl REDmod {
    /// retrieve "mods" folder
    fn try_new() -> anyhow::Result<Self> {
        let current_folder = std::env::current_exe()?;
        let redmod_folder = current_folder
            .parent()
            .context("plugins folder")?
            .parent()
            .context("red4ext folder")?
            .parent()
            .context("Cyberpunk 2077 folder")?
            .join("mods");
        if redmod_folder.is_dir() {
            return Ok(Self(redmod_folder));
        }
        anyhow::bail!("{:#?} is not a valid folder", redmod_folder.file_name());
    }
    /// retrieve all mod folders under "mods"
    fn mods(self) -> Vec<Mod> {
        std::fs::read_dir(self.0)
            .unwrap() // safety: dir already checked
            .filter_map(std::result::Result::ok)
            .filter_map(|x| {
                let path = x.path();
                if path.is_dir() {
                    return Some(Mod(path));
                }
                None
            })
            .collect()
    }
}
struct Mod(std::path::PathBuf);
impl Mod {
    /// retrieve sound bank from a mod folder, if it exists
    fn bank(&self) -> Option<Bank> {
        std::fs::read_dir(&self.0)
            // safety: dir already checked
            .unwrap()
            .filter_map(std::result::Result::ok)
            .filter(|x| x.path().is_file())
            .find(|x| is_manifest(&x.path()))
            .and_then(|x| {
                std::fs::read(x.path())
                    .ok()
                    .and_then(|x| serde_yaml::from_slice::<Bank>(x.as_slice()).ok())
            })
    }
    fn name(&self) -> &str {
        self.0.file_stem().unwrap().to_str().unwrap()
    }
}

/// check if path is valid file named "audioware" with YAML extension
fn is_manifest(file: &std::path::Path) -> bool {
    if let Some(stem) = file.file_stem() {
        let stem = stem.to_ascii_lowercase();
        if stem == "audioware" {
            if let Some(extension) = file.extension() {
                let extension = extension.to_ascii_lowercase();
                return extension == "yml" || extension == "yaml";
            }
        }
    }
    false
}

pub struct SoundBanks;
impl SoundBanks {
    pub fn initialize() -> anyhow::Result<()> {
        let redmod_folder = REDmod::try_new()?;
        let mods = redmod_folder.mods();
        if let Ok(mut guard) = BANKS.clone().try_lock() {
            *guard = Banks::from(mods.as_slice());
        }
        Ok(())
    }
    pub fn contains_sound(id: impl Into<SoundId>) -> bool {
        if let Ok(guard) = BANKS.clone().try_lock() {
            return guard.contains_sound(id.into());
        }
        false
    }
    pub fn contains_sound_for_event(event: &Ref<Event>) -> bool {
        use red4ext_rs::conv::ClassType;
        let name: Option<CName> = match event {
            v if v.is_exactly_a(CName::new("entAudioEvent")) => {
                let ent_audio_event: &Ref<AudioEvent> = unsafe { std::mem::transmute(v) };
                Some(ent_audio_event.deref().event_name.clone())
            }
            v if v.is_exactly_a(CName::new(SoundPlayEvent::NATIVE_NAME)) => {
                let sound_play_event: &Ref<SoundPlayEvent> = unsafe { std::mem::transmute(v) };
                Some(sound_play_event.deref().sound_name.clone())
            }
            _ => None,
        };
        if let Some(name) = name {
            return Self::contains_sound(name);
        }
        false
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Banks(HashMap<String, Bank>);

impl From<&[Mod]> for Banks {
    fn from(value: &[Mod]) -> Self {
        let mut banks = Self::default();
        for module in value.iter() {
            if let Some(bank) = module.bank() {
                banks.insert(module.name().to_string(), bank);
            }
        }
        banks
    }
}

impl Banks {
    fn contains_sound(&self, key: impl Into<SoundId>) -> bool {
        let key = key.into();
        for bank in self.0.values() {
            if bank.sounds.contains_key(&key) {
                return true;
            }
        }
        false
    }
    fn get_sound(&self, key: impl Into<SoundId>) -> Option<Sound> {
        let key = key.into();
        for bank in self.0.values() {
            if let Some(sound) = bank.sounds.get(&key) {
                return Some(sound.clone());
            }
        }
        None
    }
}

impl Deref for Banks {
    type Target = HashMap<String, Bank>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Banks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Bank {
    name: String,
    version: VersionReq,
    sounds: HashMap<SoundId, Sound>,
}
impl Bank {
    pub fn is_empty(&self) -> bool {
        self.sounds.is_empty()
    }
}
// impl Bank {
//     fn get(&self, sfx: impl AsRef<str>) -> Option<&dyn Audio> {
//         self.0.get(sfx.as_ref()).map(|a| a.deref())
//     }
// }

#[cfg(test)]
mod tests {
    use super::Bank;
    #[test]
    pub fn deserialize() {
        let filepath = std::path::PathBuf::from("./tests/audioware.yml");
        let yaml = std::fs::read(filepath).unwrap();
        let bank = serde_yaml::from_slice::<Bank>(yaml.as_slice());
        assert!(bank.is_ok());
    }
}