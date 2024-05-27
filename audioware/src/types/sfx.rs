use std::path::PathBuf;

use kira::sound::static_sound::StaticSoundData;
use serde::Deserialize;

/// manifest sfx upon deserialization
#[derive(Debug, Clone, Deserialize)]
pub struct Sfx(PathBuf);

/// in-memory sfx once stored
#[derive(Debug, Clone)]
pub struct InMemorySfx(StaticSoundData);

impl From<StaticSoundData> for InMemorySfx {
    fn from(value: StaticSoundData) -> Self {
        Self(value)
    }
}

impl AsRef<StaticSoundData> for InMemorySfx {
    fn as_ref(&self) -> &StaticSoundData {
        &self.0
    }
}

impl AsRef<std::path::Path> for Sfx {
    fn as_ref(&self) -> &std::path::Path {
        &self.0
    }
}
