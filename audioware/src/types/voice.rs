use std::collections::HashMap;

use audioware_types::interop::gender::PlayerGender;
use fixed_map::Map;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use red4ext_rs::types::CName;
use semver::Version;
use serde::Deserialize;
use validator::{Validate, ValidationError};
use validator::{ValidateArgs, ValidationErrors};

use crate::engine::SoundId;
use audioware_types::interop::locale::Locale;

#[derive(Debug, Clone, Deserialize)]
pub struct Voices {
    pub version: Version,
    pub voices: HashMap<SoundId, Voice>,
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

#[derive(Debug, Clone, Deserialize)]
pub struct Voice {
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
        match gender {
            PlayerGender::Female => &self.female,
            PlayerGender::Male => &self.male,
        }
    }
    pub fn subtitle(&self, locale: Locale) -> (&str, &str) {
        (
            self.female
                .get(locale)
                .map(|x| x.subtitle.as_str())
                .unwrap_or(""),
            self.male
                .get(locale)
                .map(|x| x.subtitle.as_str())
                .unwrap_or(""),
        )
    }
}

impl<'v_a> ValidateArgs<'v_a> for Voice {
    type Args = &'v_a std::path::Path;

    fn validate_args(&self, args: Self::Args) -> Result<(), validator::ValidationErrors> {
        let mut errors = ValidationErrors::new();
        for audio in self.female.values() {
            if let Err(e) = validate_static_sound_data(&audio.file, args) {
                errors.add("female", e);
            }
        }
        for audio in self.male.values() {
            if let Err(e) = validate_static_sound_data(&audio.file, args) {
                errors.add("male", e);
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct AudioSubtitle {
    #[validate(custom(function = "validate_static_sound_data", arg = "&'v_a std::path::Path"))]
    pub file: std::path::PathBuf,
    pub subtitle: String,
}

fn validate_static_sound_data(
    value: &std::path::PathBuf,
    arg: &std::path::Path,
) -> Result<(), ValidationError> {
    let arg = std::fs::canonicalize(arg).unwrap();
    let path = arg.join(value);
    if !path.starts_with(arg) {
        return Err(ValidationError::new("file located outside of mod folder"));
    }
    StaticSoundData::from_file(path, StaticSoundSettings::default())
        .map(|_| ())
        .map_err(|e| {
            red4ext_rs::error!("{:#?}", e);
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
            file: "en-us/v_sq017_f_19795c050029f000.Wav".into(),
            subtitle: "Again?".to_string(),
        };
        let validation = audio.validate_args(folder.as_path());
        assert!(validation.is_ok());

        let audio = AudioSubtitle {
            file: "en-us/../../v_sq017_f_19795c050029f000.Wav".into(),
            subtitle: "Again?".to_string(),
        };
        let validation = audio.validate_args(folder.as_path());
        assert!(validation.is_err());
    }
}
