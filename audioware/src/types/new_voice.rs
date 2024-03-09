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
pub struct LocalizedVoices(Map<Locale, Voice>);

#[derive(Debug, Clone, Deserialize)]
pub enum Voice {
    Neutral(NeutralVoice),
    Genderized(GenderizedVoice),
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct NeutralVoice {
    #[validate(custom(function = "validate_static_sound_data", arg = "&'v_a std::path::Path"))]
    female: Option<std::path::PathBuf>,
    #[validate(custom(function = "validate_static_sound_data", arg = "&'v_a std::path::Path"))]
    male: Option<std::path::PathBuf>,
    subtitle: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GenderizedVoice {
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
