use std::collections::HashMap;

use semver::Version;
use serde::Deserialize;

use super::id::SfxId;

#[derive(Debug, Clone, Deserialize)]
pub struct Sfxs {
    #[allow(dead_code)]
    pub version: Version,
    pub sfx: HashMap<SfxId, Sfx>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Sfx(std::path::PathBuf);
