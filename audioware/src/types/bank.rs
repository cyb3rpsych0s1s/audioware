use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use audioware_sys::interop::{gender::PlayerGender, locale::Locale};
use kira::sound::static_sound::StaticSoundData;

use red4ext_rs::types::CName;
use semver::Version;
use snafu::ResultExt;

use super::{
    error::{BankError, CannotReadDirSnafu, CannotReadManifestSnafu, Error, InvalidManifestSnafu},
    id::{Id, SfxId, VoiceId},
    manifest::Manifest,
    redmod::{Mod, ModName},
    sfx::InMemorySfx,
    voice::{validate_static_sound_data, AudioSubtitle, DualVoice, Voice},
};

#[derive(Debug)]
pub struct Bank {
    r#mod: ModName,
    #[allow(dead_code)]
    version: Version,
    voices: Option<HashMap<VoiceId, Voice>>,
    sfx: Option<HashMap<SfxId, InMemorySfx>>,
    folder: std::path::PathBuf,
    #[allow(dead_code)]
    filename: String,
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
            voices.values_mut().for_each(|voice| match voice {
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
    }
    pub fn retain_unique_ids(&mut self, ids: &Mutex<HashSet<Id>>) {
        if let Some(voices) = &mut self.voices {
            voices.retain(|id, _| {
                if let Ok(mut guard) = ids.try_lock() {
                    let inserted = guard.insert(Id::from(id));
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
            sfx.retain(|id, _| {
                if let Ok(mut guard) = ids.try_lock() {
                    let inserted = guard.insert(Id::from(id));
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
        if let Some(sfx) = self.sfx.as_ref().and_then(|x| x.get(id)) {
            return Some(sfx.as_ref().clone());
        }
        None
    }
    pub fn data_from_voice_id(
        &self,
        gender: PlayerGender,
        language: Locale,
        id: &VoiceId,
    ) -> Option<StaticSoundData> {
        if let Some(voice) = self.voices.as_ref().and_then(|x| x.get(id)) {
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
        }
    }
    pub fn voices(&self) -> Option<&HashMap<VoiceId, Voice>> {
        self.voices.as_ref()
    }
}

impl TryFrom<&Mod> for Vec<Bank> {
    type Error = Error;

    fn try_from(value: &Mod) -> Result<Self, Self::Error> {
        // safety: dir already checked
        if let Ok(entry) = std::fs::read_dir(value) {
            let files = entry
                .filter_map(std::result::Result::ok)
                .filter(|x| x.path().is_file())
                .map(|x| x.path())
                .collect::<Vec<_>>();
            let mut voices: Option<HashMap<VoiceId, Voice>> = None;
            let mut sfx: Option<HashMap<SfxId, InMemorySfx>> = None;
            let mut banks = Vec::with_capacity(files.len());
            for file in files {
                if is_manifest(&file) {
                    let content = std::fs::read(&file).context(CannotReadManifestSnafu {
                        path: file.as_path().display().to_string(),
                    })?;
                    let manifest = serde_yaml::from_slice::<Manifest>(content.as_slice()).context(
                        InvalidManifestSnafu {
                            path: file.as_path().display().to_string(),
                        },
                    )?;
                    if manifest.voices.is_none() && manifest.sfx.is_none() {
                        return Err(BankError::Empty {
                            filename: file.display().to_string(),
                        }
                        .into());
                    }
                    if let Some(found) = manifest.voices {
                        voices = Some(found);
                    }
                    if let Some(found) = manifest.sfx {
                        let mut in_memory: HashMap<SfxId, InMemorySfx> =
                            HashMap::with_capacity(found.len());
                        for (k, v) in found.into_iter() {
                            match StaticSoundData::from_file(v.as_ref()) {
                                Ok(data) => {
                                    in_memory.insert(k, data.into());
                                }
                                Err(_) => {
                                    red4ext_rs::error!(
                                        "unable to load audio in memory, skipping... ({})",
                                        v.as_ref().display()
                                    );
                                }
                            }
                        }
                        sfx = Some(in_memory);
                    }
                    banks.push(Bank {
                        r#mod: value.name(),
                        version: manifest.version,
                        voices: voices.to_owned(),
                        sfx: sfx.to_owned(),
                        folder: value.as_ref().to_owned(),
                        filename: file
                            .file_name()
                            .expect("yaml file has already been read")
                            .to_string_lossy()
                            .to_string(),
                    });
                }
            }
        }
        Err(CannotReadDirSnafu {
            path: value.as_ref().display().to_string(),
        }
        .build()
        .into())
    }
}

#[inline]
fn is_manifest(file: &std::path::Path) -> bool {
    file.extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(|x| x == "yml" || x == "yaml")
        .unwrap_or(false)
}
