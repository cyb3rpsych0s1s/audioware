use std::collections::HashMap;

use audioware_sys::interop::gender::PlayerGender;
use fixed_map::Map;
use kira::sound::static_sound::StaticSoundData;
use red4ext_rs::types::CName;
use semver::Version;
use serde::Deserialize;
use validator::{Validate, ValidationError};
use validator::{ValidateArgs, ValidationErrors};

use audioware_sys::interop::locale::Locale;

use super::id::VoiceId;

#[derive(Debug, Clone, Deserialize)]
pub struct Voices {
    #[allow(dead_code)]
    pub version: Version,
    pub voices: HashMap<VoiceId, Voice>,
}

impl Voices {
    pub fn subtitles(&self, locale: Locale) -> Vec<Subtitle<'_>> {
        self.voices
            .iter()
            .map(|(id, voice)| {
                let (female, male) = voice.subtitle(locale);
                Subtitle {
                    key: id.as_ref(),
                    female,
                    male,
                }
            })
            .collect::<Vec<_>>()
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    /// for voice that support both gender, e.g. V's
    Dual(DualVoice),
    /// for voice that only ever have one gender, e.g. Judy's
    Single(Map<Locale, AudioSubtitle>),
}

#[derive(Debug, Clone, Deserialize)]
pub struct DualVoice {
    #[serde(rename = "fem")]
    pub female: Map<Locale, AudioSubtitle>,
    pub male: Map<Locale, AudioSubtitle>,
}

pub struct Subtitle<'a> {
    pub key: &'a CName,
    pub female: &'a str,
    pub male: &'a str,
}

impl Voice {
    pub fn audios(&self, gender: &PlayerGender) -> &Map<Locale, AudioSubtitle> {
        match self {
            Voice::Dual(DualVoice { female, male }) => match gender {
                PlayerGender::Female => female,
                PlayerGender::Male => male,
            },
            Voice::Single(voice) => voice,
        }
    }
    pub fn subtitle(&self, locale: Locale) -> (&str, &str) {
        match self {
            Voice::Dual(DualVoice { female, male }) => (
                female
                    .get(locale)
                    .map(|x| x.subtitle.as_str())
                    .unwrap_or(""),
                male.get(locale).map(|x| x.subtitle.as_str()).unwrap_or(""),
            ),
            Voice::Single(voice) => (
                voice.get(locale).map(|x| x.subtitle.as_str()).unwrap_or(""),
                voice.get(locale).map(|x| x.subtitle.as_str()).unwrap_or(""),
            ),
        }
    }
}

impl<'v_a> ValidateArgs<'v_a> for Voice {
    type Args = &'v_a std::path::Path;

    fn validate_with_args(&self, args: Self::Args) -> Result<(), validator::ValidationErrors> {
        let mut errors = ValidationErrors::new();
        match self {
            Voice::Dual(DualVoice { female, male }) => {
                for audio in female.values() {
                    if let Some(file) = &audio.file {
                        if let Err(e) = validate_static_sound_data(file, args) {
                            errors.add("female", e);
                        }
                    }
                }
                for audio in male.values() {
                    if let Some(file) = &audio.file {
                        if let Err(e) = validate_static_sound_data(file, args) {
                            errors.add("male", e);
                        }
                    }
                }
            }
            Voice::Single(voice) => {
                for audio in voice.values() {
                    if let Some(file) = &audio.file {
                        if let Err(e) = validate_static_sound_data(file, args) {
                            errors.add("uni", e);
                        }
                    }
                }
            }
        };
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[validate(context = "std::path::Path")]
pub struct AudioSubtitle {
    #[validate(custom(function = "validate_static_sound_data", use_context))]
    pub file: Option<std::path::PathBuf>,
    pub subtitle: String,
}

pub fn validate_static_sound_data(
    value: &std::path::PathBuf,
    arg: &std::path::Path,
) -> Result<(), ValidationError> {
    let arg = std::fs::canonicalize(arg).unwrap();
    let path = arg.join(value);
    if !path.starts_with(arg) {
        return Err(ValidationError::new("file located outside of mod folder"));
    }
    StaticSoundData::from_file(path).map(|_| ()).map_err(|e| {
        red4ext_rs::error!("{:#?} ({})", e, value.display());
        ValidationError::new("invalid audio file")
    })
}

#[cfg(test)]
mod tests {
    use validator::ValidateArgs;

    use crate::types::voice::{AudioSubtitle, Voices};

    #[test]
    pub fn deserialize() {
        let filepath = std::path::PathBuf::from("./tests/voices.yml");
        let yaml = std::fs::read(filepath).unwrap();
        let voices = serde_yaml::from_slice::<Voices>(yaml.as_slice());
        assert!(voices.is_ok());
    }

    #[test]
    pub fn validate() {
        let folder = std::path::PathBuf::from("./tests");
        let audio = AudioSubtitle {
            file: Some("en-us/v_sq017_f_19795c050029f000.Wav".into()),
            subtitle: "Again?".to_string(),
        };
        let validation = audio.validate_with_args(folder.as_path());
        assert!(validation.is_ok());

        let audio = AudioSubtitle {
            file: Some("en-us/../../v_sq017_f_19795c050029f000.Wav".into()),
            subtitle: "Again?".to_string(),
        };
        let validation = audio.validate_with_args(folder.as_path());
        assert!(validation.is_err());
    }
}
