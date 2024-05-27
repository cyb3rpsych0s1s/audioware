use std::collections::HashMap;

use kira::sound::static_sound::StaticSoundData;
use serde::Deserialize;

use super::id::SfxId;

#[derive(Debug, Clone, Deserialize)]
pub struct Sfx(std::path::PathBuf);

impl AsRef<std::path::Path> for Sfx {
    fn as_ref(&self) -> &std::path::Path {
        &self.0
    }
}
