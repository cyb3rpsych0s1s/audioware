use std::collections::HashMap;

use semver::Version;
use serde::Deserialize;

use super::{
    id::{SfxId, VoiceId},
    sfx::Sfx,
    voice::Voice,
};

/// manifest is the file modder use to describe their sound
#[derive(Debug, Deserialize)]
pub struct Manifest {
    #[allow(dead_code)]
    pub version: Version,
    pub voices: Option<HashMap<VoiceId, Voice>>,
    pub sfx: Option<HashMap<SfxId, Sfx>>,
}
