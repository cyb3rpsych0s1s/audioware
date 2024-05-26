use std::collections::HashMap;

use kira::sound::static_sound::StaticSoundData;
use semver::Version;
use serde::Deserialize;

use super::id::SfxId;

#[derive(Debug, Deserialize)]
pub struct Sfxs {
    #[allow(dead_code)]
    pub version: Version,
    pub sfx: HashMap<SfxId, Sfx>,
}

#[derive(Debug)]
pub struct InMemorySfxs {
    #[allow(dead_code)]
    pub version: Version,
    pub sfx: HashMap<SfxId, StaticSoundData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Sfx(std::path::PathBuf);

impl AsRef<std::path::Path> for Sfx {
    fn as_ref(&self) -> &std::path::Path {
        &self.0
    }
}
