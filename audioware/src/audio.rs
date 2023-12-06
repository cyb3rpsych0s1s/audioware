use std::{path::Path, time::Duration, collections::HashMap};

use kira::sound::static_sound::StaticSoundData;
use red4ext_rs::types::CName;
use serde::Deserialize;
use std::fmt::Debug;

use crate::{locale::Locale, gender::Gender};

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

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct StaticAudio {
    file: std::path::PathBuf,
    #[serde(skip)]
    data: Option<StaticSoundData>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Translation {
    Simple(StaticAudio),
    Subtitle{
        file: StaticAudio,
        subtitle: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Sound {
    Simple(StaticAudio),
    Gender{
        #[serde(flatten)]
        gender: HashMap<Gender, HashMap<Locale, Translation>>,
        kind: Option<Kind>,
    },
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SoundId(String);

impl From<CName> for SoundId {
    fn from(value: CName) -> Self {
        Self(red4ext_rs::ffi::resolve_cname(&value).to_string())
    }
}
