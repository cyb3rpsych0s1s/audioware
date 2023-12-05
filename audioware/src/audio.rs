use std::{path::Path, time::Duration};

use serde::Deserialize;
use std::fmt::Debug;

use crate::locale::Locale;

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
