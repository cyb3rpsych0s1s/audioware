use std::{
    borrow::BorrowMut,
    collections::HashSet,
    sync::{Arc, Mutex},
};

use audioware_sys::interop::{gender::PlayerGender, locale::Locale};
use kira::sound::static_sound::StaticSoundData;

use red4ext_rs::types::CName;
use serde::Deserialize;

use crate::types::voice::{validate_static_sound_data, AudioSubtitle};

use super::{
    id::VoiceId,
    redmod::{Mod, ModName, REDmod},
    voice::{DualVoice, Voices},
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
        REDmod::try_new().unwrap().as_ref().join(&self.r#mod)
    }
    pub fn retain_valid_audio(&mut self) {
        let folder = self.folder();
        self.voices
            .voices
            .values_mut()
            .for_each(|voice| match voice {
                super::voice::Voice::Dual(DualVoice { female, male }) => {
                    for audio in female.values_mut().chain(male.values_mut()) {
                        if let Some(file) = audio.file.clone() {
                            if validate_static_sound_data(&file, &folder).is_err() {
                                audio.file = None;
                            }
                        }
                    }
                }
                super::voice::Voice::Single(voice) => {
                    for audio in voice.values_mut() {
                        if let Some(file) = audio.file.clone() {
                            if validate_static_sound_data(&file, &folder).is_err() {
                                audio.file = None;
                            }
                        }
                    }
                }
            });
    }
    pub fn retain_unique_ids(&mut self, ids: &Arc<Mutex<HashSet<VoiceId>>>) {
        self.voices.voices.retain(|id, _| {
            if let Ok(mut guard) = ids.clone().borrow_mut().try_lock() {
                let inserted = guard.insert(id.clone());
                if !inserted {
                    red4ext_rs::error!("duplicate sound id ({id})");
                }
                return inserted;
            } else {
                red4ext_rs::error!("unable to reach sound ids");
            }
            false
        });
    }
    pub fn data(
        &self,
        gender: PlayerGender,
        language: Locale,
        id: &CName,
    ) -> Option<StaticSoundData> {
        if let Some(voice) = self.voices.voices.get(&id.clone().into()) {
            let audios = voice.audios(&gender);
            if let Some(AudioSubtitle {
                file: Some(file),
                settings,
                ..
            }) = audios.get(language)
            {
                return StaticSoundData::from_file(
                    self.folder().join(file),
                    settings.clone().map(Into::into).unwrap_or_default(),
                )
                .ok();
            }
        }
        None
    }
    pub fn voices(&self) -> &Voices {
        &self.voices
    }
}

impl TryFrom<&Mod> for Bank {
    type Error = anyhow::Error;

    fn try_from(value: &Mod) -> Result<Self, Self::Error> {
        // safety: dir already checked
        if let Ok(entry) = std::fs::read_dir(value) {
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
    file.file_stem()
        .and_then(std::ffi::OsStr::to_str)
        .map(|x| x == "voices")
        .unwrap_or(false)
        && file
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .map(|x| x == "yml" || x == "yaml")
            .unwrap_or(false)
}
