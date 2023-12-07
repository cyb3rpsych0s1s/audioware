use std::{collections::HashMap, path::Path, time::Duration};

use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use red4ext_rs::types::CName;
use serde::Deserialize;
use std::fmt::Debug;

use crate::{gender::Gender, locale::Locale};

pub trait Audio {
    fn filepath(&self) -> Path;
    fn kind(&self) -> Kind;
    fn duration(&self) -> Duration;
}

pub trait SubtitledAudio: Audio {
    fn subtitle(&self, locale: Locale) -> &str;
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Ono,
    Voice,
    Thought,
    Ambient,
    Music,
    Spoken,
}

pub trait Load {
    fn load(&mut self) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct StaticAudio {
    file: std::path::PathBuf,
    #[serde(skip)]
    pub(crate) data: Option<StaticSoundData>,
    #[serde(skip)]
    duration: Duration,
    #[serde(skip)]
    size: u64,
}

impl Load for StaticAudio {
    fn load(&mut self) -> anyhow::Result<()> {
        self.size = self
            .file
            .metadata()
            .as_ref()
            .map(std::fs::Metadata::len)
            .unwrap_or(0);
        if self.size == 0 {
            anyhow::bail!("{} size is zero byte.", self.file.display());
        }
        let data = StaticSoundData::from_file(&self.file, StaticSoundSettings::default())?;
        self.duration = data.duration();
        self.data = Some(data);
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Translation {
    Simple(StaticAudio),
    Subtitle { file: StaticAudio, subtitle: String },
}

impl Load for Translation {
    fn load(&mut self) -> anyhow::Result<()> {
        match self {
            Translation::Simple(audio) => audio.load(),
            Translation::Subtitle { file, .. } => file.load(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Sound {
    Simple(StaticAudio),
    Gender {
        #[serde(flatten)]
        gender: HashMap<Gender, HashMap<Locale, Translation>>,
        kind: Option<Kind>,
    },
}

impl Load for Sound {
    fn load(&mut self) -> anyhow::Result<()> {
        match self {
            Sound::Simple(audio) => audio.load(),
            Sound::Gender { gender, .. } => {
                for translations in gender.values_mut() {
                    for translation in translations.values_mut() {
                        translation.load()?;
                    }
                }
                Ok(())
            }
        }
    }
}

impl Sound {
    pub fn audio(&self) -> StaticAudio {
        match self {
            Sound::Simple(audio) => audio.clone(),
            Sound::Gender { gender: _, kind: _ } => todo!(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SoundId(String);

impl From<CName> for SoundId {
    fn from(value: CName) -> Self {
        Self(red4ext_rs::ffi::resolve_cname(&value).to_string())
    }
}

impl std::fmt::Display for SoundId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
