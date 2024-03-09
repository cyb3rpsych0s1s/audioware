#![allow(dead_code)]

use std::collections::HashMap;

use fixed_map::Map;

use semver::Version;
use serde::Deserialize;
use validator::Validate;

use crate::engine::SoundId;
use crate::types::voice::validate_static_sound_data;
use audioware_sys::interop::locale::Locale;

use super::voice::{AudioSubtitle, Subtitle};

#[derive(Debug, Clone, Deserialize)]
pub struct Voices {
    pub version: Version,
    pub voices: HashMap<SoundId, LocalizedVoices>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct LocalizedVoices(pub Map<Locale, Voice>);

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Neutral(NeutralVoice),
    Genderized(GenderizedVoice),
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct NeutralVoice {
    #[validate(custom(function = "validate_static_sound_data", arg = "&'v_a std::path::Path"))]
    #[serde(rename = "fem")]
    female: Option<std::path::PathBuf>,
    #[validate(custom(function = "validate_static_sound_data", arg = "&'v_a std::path::Path"))]
    male: Option<std::path::PathBuf>,
    subtitle: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GenderizedVoice {
    #[serde(rename = "fem")]
    female: AudioSubtitle,
    male: AudioSubtitle,
}

impl Voices {
    pub fn subtitles(&self, locale: Locale) -> Vec<Subtitle<'_>> {
        self.voices
            .iter()
            .filter_map(|(id, voice)| {
                voice.0.get(locale).map(|x| match x {
                    Voice::Neutral(x) => Subtitle {
                        key: id.as_ref(),
                        female: &x.subtitle,
                        male: &x.subtitle,
                    },
                    Voice::Genderized(x) => {
                        let (female, male) = x.subtitle();
                        Subtitle {
                            key: id.as_ref(),
                            female,
                            male,
                        }
                    }
                })
            })
            .collect()
    }
}

impl GenderizedVoice {
    pub fn subtitle(&self) -> (&str, &str) {
        (&self.female.subtitle, &self.male.subtitle)
    }
}

#[cfg(test)]
mod tests {
    use validator::ValidateArgs;

    use crate::types::new_voice::{NeutralVoice, Voices};

    #[test]
    pub fn deserialize() {
        let filepath = std::path::PathBuf::from("./tests/voices2.yml");
        let yaml = std::fs::read(filepath).unwrap();
        let voices = serde_yaml::from_slice::<Voices>(yaml.as_slice());
        assert!(voices.is_ok());
    }

    #[test]
    pub fn validate() {
        let folder = std::path::PathBuf::from("./tests");
        let audio = NeutralVoice {
            female: Some("en-us/v_sq017_f_19795c050029f000.Wav".into()),
            male: Some("en-us/v_sq017_f_19795c050029f000.Wav".into()),
            subtitle: "Again?".to_string(),
        };
        let validation = audio.validate_args((folder.as_path(), folder.as_path()));
        assert!(validation.is_ok());

        let audio = NeutralVoice {
            female: Some("en-us/../../v_sq017_f_19795c050029f000.Wav".into()),
            male: Some("en-us/../../v_sq017_f_19795c050029f000.Wav".into()),
            subtitle: "Again?".to_string(),
        };
        let validation = audio.validate_args((folder.as_path(), folder.as_path()));
        assert!(validation.is_err());
    }
}
