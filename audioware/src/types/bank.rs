use std::{collections::HashSet, sync::Mutex};

use audioware_sys::interop::{gender::PlayerGender, locale::Locale};
use kira::sound::static_sound::StaticSoundData;

use red4ext_rs::types::CName;
use snafu::ResultExt;

use crate::types::{
    error::{UnableToDeserializeSnafu, UnableToReadManifestSnafu},
    voice::{validate_static_sound_data, AudioSubtitle},
};

use super::{
    error::{Error, UnableToReadDirSnafu},
    id::{Id, SfxId, VoiceId},
    redmod::{Mod, ModName},
    sfx::Sfxs,
    voice::{DualVoice, Voices},
};

#[derive(Debug, Clone)]
pub struct Bank {
    r#mod: ModName,
    voices: Option<Voices>,
    sfx: Option<Sfxs>,
    folder: std::path::PathBuf,
}

impl Bank {
    pub fn name(&self) -> &ModName {
        &self.r#mod
    }
    pub fn folder(&self) -> std::path::PathBuf {
        self.folder.clone()
    }
    pub fn retain_valid_audio(&mut self) {
        let folder = self.folder();
        if let Some(voices) = &mut self.voices {
            voices.voices.values_mut().for_each(|voice| match voice {
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
                                red4ext_rs::error!("invalid voice audio file ({})", file.display());
                            }
                        }
                    }
                }
            });
        }
        if let Some(sfx) = &mut self.sfx {
            sfx.sfx.retain(|_, v| {
                let valid = validate_static_sound_data(&v.0, &folder).is_ok();
                if valid {
                    red4ext_rs::error!("invalid sfx audio file ({})", v.0.display());
                }
                valid
            });
        }
    }
    pub fn retain_unique_ids(&mut self, ids: &Mutex<HashSet<Id>>) {
        if let Some(voices) = &mut self.voices {
            voices.voices.retain(|id, _| {
                if let Ok(mut guard) = ids.try_lock() {
                    let inserted = guard.insert(Id::Voice(id.clone()));
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
        if let Some(sfx) = &mut self.sfx {
            sfx.sfx.retain(|id, _| {
                if let Ok(mut guard) = ids.try_lock() {
                    let inserted = guard.insert(Id::Sfx(id.clone()));
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
    }
    pub fn data_from_sfx_id(&self, id: &SfxId) -> Option<StaticSoundData> {
        if let Some(sfx) = self.sfx.as_ref().and_then(|x| x.sfx.get(id)) {
            return StaticSoundData::from_file(self.folder().join(sfx.0.as_path())).ok();
        }
        None
    }
    pub fn data_from_voice_id(
        &self,
        gender: PlayerGender,
        language: Locale,
        id: &VoiceId,
    ) -> Option<StaticSoundData> {
        if let Some(voice) = self.voices.as_ref().and_then(|x| x.voices.get(id)) {
            let audios = voice.audios(&gender);
            if let Some(AudioSubtitle {
                file: Some(file), ..
            }) = audios.get(language)
            {
                return StaticSoundData::from_file(self.folder().join(file)).ok();
            }
        }
        None
    }
    pub fn data_from_any_id(
        &self,
        gender: PlayerGender,
        language: Locale,
        id: &CName,
    ) -> Result<StaticSoundData, Error> {
        match crate::engine::banks::typed_id(id)? {
            Id::Voice(id) => Ok(self.data_from_voice_id(gender, language, &id).unwrap()),
            Id::Sfx(id) => Ok(self.data_from_sfx_id(&id).unwrap()),
            Id::Any(_) => unreachable!("method typed_id should only return valid and typed ID"),
        }
    }
    pub fn voices(&self) -> Option<&Voices> {
        self.voices.as_ref()
    }
}

impl TryFrom<&Mod> for Bank {
    type Error = Error;

    fn try_from(value: &Mod) -> Result<Self, Self::Error> {
        // safety: dir already checked
        if let Ok(entry) = std::fs::read_dir(value) {
            let files = entry
                .filter_map(std::result::Result::ok)
                .filter(|x| x.path().is_file())
                .map(|x| x.path())
                .collect::<Vec<_>>();
            let mut voices = None;
            let mut sfx = None;
            if let Some(manifest) = files.iter().find(|x| is_voices_manifest(x)) {
                let content = std::fs::read(manifest).context(UnableToReadManifestSnafu {
                    path: manifest.as_path().display().to_string(),
                })?;
                let entries = serde_yaml::from_slice::<Voices>(content.as_slice()).context(
                    UnableToDeserializeSnafu {
                        path: manifest.as_path().display().to_string(),
                        kind: "voices",
                    },
                )?;
                voices = Some(entries);
            }
            if let Some(manifest) = files.iter().find(|x| is_sfx_manifest(x)) {
                let content = std::fs::read(manifest).context(UnableToReadManifestSnafu {
                    path: manifest.as_path().display().to_string(),
                })?;
                let entries = serde_yaml::from_slice::<Sfxs>(content.as_slice()).context(
                    UnableToDeserializeSnafu {
                        path: manifest.as_path().display().to_string(),
                        kind: "sfx",
                    },
                )?;
                sfx = Some(entries);
            }
            return Ok(Self {
                r#mod: value.name(),
                voices,
                sfx,
                folder: value.as_ref().to_owned(),
            });
        }
        Err(UnableToReadDirSnafu {
            path: value.as_ref().display().to_string(),
        }
        .build()
        .into())
    }
}

#[inline]
fn is_manifest(file: &std::path::Path, stem: &str) -> bool {
    file.file_stem()
        .and_then(std::ffi::OsStr::to_str)
        .map(|x| x == stem)
        .unwrap_or(false)
        && file
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .map(|x| x == "yml" || x == "yaml")
            .unwrap_or(false)
}

/// check if path is valid file named "voices" with YAML extension
fn is_voices_manifest(file: &std::path::Path) -> bool {
    is_manifest(file, "voices")
}

/// check if path is valid file named "sfx" with YAML extension
fn is_sfx_manifest(file: &std::path::Path) -> bool {
    is_manifest(file, "sfx")
}
