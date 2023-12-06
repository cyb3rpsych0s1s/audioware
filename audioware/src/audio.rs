use std::{path::Path, time::Duration, collections::HashMap};

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
pub struct SubtitledSound {
    file: std::path::PathBuf,
    subtitle: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Localization {
    Simple(std::path::PathBuf),
    Subtitled(SubtitledSound),
}

#[derive(Debug, Clone, Deserialize)]
pub struct VoiceSound {
    #[serde(flatten)]
    gender: HashMap<Gender, HashMap<Locale, Localization>>,
    kind: Option<Kind>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Sound {
    Simple(std::path::PathBuf),
    Genderized(VoiceSound),
}

#[derive(Debug, Clone, Deserialize)]
pub enum Subtitle {}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SoundId(String);

impl From<CName> for SoundId {
    fn from(value: CName) -> Self {
        Self(red4ext_rs::ffi::resolve_cname(&value).to_string())
    }
}
