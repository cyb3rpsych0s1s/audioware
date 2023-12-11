use std::collections::HashMap;

use fixed_map::Map;
use semver::Version;
use serde::Deserialize;

use crate::engine::id::SoundId;
use audioware_types::interop::locale::Locale;

#[derive(Debug, Clone, Deserialize)]
pub struct Voices {
    version: Version,
    voices: HashMap<SoundId, Voice>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Voice {
    #[serde(rename = "fem")]
    female: Map<Locale, AudioSubtitle>,
    male: Map<Locale, AudioSubtitle>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AudioSubtitle {
    file: std::path::PathBuf,
    subtitle: String,
}

#[cfg(test)]
mod tests {
    use crate::types::voice::Voices;

    #[test]
    pub fn deserialize() {
        let filepath = std::path::PathBuf::from("./tests/voices.yml");
        let yaml = std::fs::read(filepath).unwrap();
        let voices = serde_yaml::from_slice::<Voices>(yaml.as_slice());
        println!("{voices:#?}");
        assert!(voices.is_ok());
    }
}
