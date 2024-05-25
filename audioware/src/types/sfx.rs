use std::collections::HashMap;

use red4ext_rs::types::CName;
use semver::Version;
use serde::Deserialize;

use super::{id::SfxId, GetRaw};

#[derive(Debug, Clone, Deserialize)]
pub struct Sfxs {
    #[allow(dead_code)]
    pub version: Version,
    pub sfx: HashMap<SfxId, Sfx>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Sfx(pub std::path::PathBuf);

impl GetRaw for HashMap<SfxId, Sfx> {
    type Output = Sfx;
    fn get_raw(&self, raw: &CName) -> Option<&Self::Output> {
        for (k, v) in self.iter() {
            if k.as_ref() == raw {
                return Some(v);
            }
        }
        None
    }
}
