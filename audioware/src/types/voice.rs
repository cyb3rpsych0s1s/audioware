use std::collections::HashMap;

use fixed_map::Map;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use semver::Version;
use serde::Deserialize;
use validator::{Validate, ValidationError};

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

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct AudioSubtitle {
    #[validate(custom(function = "validate_static_sound_data", arg = "&'v_a std::path::Path"))]
    file: std::path::PathBuf,
    subtitle: String,
}

fn validate_static_sound_data(file: &std::path::PathBuf, folder: &std::path::Path) -> Result<(), ValidationError> {
    let path = folder.join(file);
    StaticSoundData::from_file(path, StaticSoundSettings::default())
        .map(|_| ())
        .map_err(|e| {
            println!("{:#?}", e);
            ValidationError::new("invalid file")
        })
}

#[cfg(test)]
mod tests {
    use validator::ValidateArgs;

    use crate::types::voice::{Voices, AudioSubtitle};

    #[test]
    pub fn deserialize() {
        let filepath = std::path::PathBuf::from("./tests/voices.yml");
        let yaml = std::fs::read(filepath).unwrap();
        let voices = serde_yaml::from_slice::<Voices>(yaml.as_slice());
        assert!(voices.is_ok());
    }

    #[test]
    pub fn validate() {
        let folder = std::path::PathBuf::from("./tests/en-us");
        let audio = AudioSubtitle {
            file: "v_sq017_f_19795c050029f000.Wav".into(),
            subtitle: "Again?".to_string()
        };
        let validation = audio.validate_args(folder.as_path());
        assert!(validation.is_ok());
    }
}
