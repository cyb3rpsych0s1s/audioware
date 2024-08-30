//! Manifest definitions.

use std::{collections::HashMap, fmt, hash::Hash, path::PathBuf};

use crate::{PlayerGender, ScnDialogLineType};
use semver::Version;
use serde::Deserialize;

#[doc(hidden)]
mod jingle;
mod music;
mod ono;
#[doc(hidden)]
mod playlist;
mod setting;
mod sfx;
mod voice;

#[doc(hidden)]
pub use jingle::*;
pub use music::*;
pub use ono::*;
#[doc(hidden)]
pub use playlist::*;
pub use setting::*;
pub use sfx::*;
pub use voice::*;

/// allows modder to describe audio files, subtitles and settings.
#[derive(Deserialize)]
pub struct Manifest {
    pub version: Version,
    pub sfx: Option<HashMap<String, Sfx>>,
    pub onos: Option<HashMap<String, Ono>>,
    pub voices: Option<HashMap<String, Voice>>,
    pub music: Option<HashMap<String, Music>>,
    #[doc(hidden)]
    pub playlist: Option<HashMap<String, Playlist>>,
    #[doc(hidden)]
    pub jingles: Option<HashMap<String, Jingle>>,
}

// until proper implementations for 'playlist' and 'jingles' are added
impl fmt::Debug for Manifest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Manifest")
            .field("version", &self.version)
            .field("sfx", &self.sfx)
            .field("onos", &self.onos)
            .field("voices", &self.voices)
            .field("music", &self.music)
            .finish_non_exhaustive()
    }
}

/// [Audio] with optional [Usage].
#[derive(Debug, Deserialize)]
pub struct UsableAudio {
    #[serde(flatten)]
    pub audio: Audio,
    pub usage: Option<Usage>,
}

/// Audio file path with optional [Settings].
#[derive(Debug, Deserialize, Clone)]
pub struct Audio {
    pub file: PathBuf,
    pub settings: Option<Settings>,
}

/// Gender-based audio resource.
#[derive(Debug, Deserialize, Clone)]
pub struct GenderBased<T> {
    pub fem: T,
    pub male: T,
}

impl<T> GenderBased<T> {
    pub fn iter(&self) -> GenderBasedIterator<T> {
        GenderBasedIterator {
            entries: self,
            index: 0,
        }
    }
    pub fn get(&self, k: &PlayerGender) -> Option<&T> {
        match k {
            PlayerGender::Female => Some(&self.fem),
            PlayerGender::Male => Some(&self.male),
        }
    }
}

pub struct GenderBasedIterator<'a, T> {
    entries: &'a GenderBased<T>,
    index: usize,
}

impl<'a, T> Iterator for GenderBasedIterator<'a, T> {
    type Item = (PlayerGender, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 2 {
            if self.index == 0 {
                self.index += 1;
                return Some((PlayerGender::Female, &self.entries.fem));
            } else {
                self.index += 1;
                return Some((PlayerGender::Male, &self.entries.male));
            }
        }
        None
    }
}

impl From<GenderBased<PathBuf>> for GenderBased<Audio> {
    fn from(value: GenderBased<PathBuf>) -> Self {
        Self {
            fem: Audio {
                file: value.fem,
                settings: None,
            },
            male: Audio {
                file: value.male,
                settings: None,
            },
        }
    }
}

impl<T> From<HashMap<PlayerGender, T>> for GenderBased<T>
where
    T: Clone,
{
    fn from(value: HashMap<PlayerGender, T>) -> Self {
        Self {
            fem: value
                .get(&PlayerGender::Female)
                .expect("female-based resource")
                .clone(),
            male: value
                .get(&PlayerGender::Male)
                .expect("male-based resource")
                .clone(),
        }
    }
}

impl From<(HashMap<PlayerGender, PathBuf>, Option<Settings>)> for GenderBased<Audio> {
    fn from((values, settings): (HashMap<PlayerGender, PathBuf>, Option<Settings>)) -> Self {
        Self {
            fem: Audio {
                file: values
                    .get(&PlayerGender::Female)
                    .expect("female-based resource")
                    .clone(),
                settings: settings.clone(),
            },
            male: Audio {
                file: values
                    .get(&PlayerGender::Male)
                    .expect("male-based resource")
                    .clone(),
                settings,
            },
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AnyAudio {
    Inline(PathBuf),
    Nested(Audio),
}

impl From<AnyAudio> for Audio {
    fn from(value: AnyAudio) -> Self {
        match value {
            AnyAudio::Inline(file) => Self {
                file,
                settings: None,
            },
            AnyAudio::Nested(audio) => audio,
        }
    }
}

impl From<(AnyAudio, Option<&Settings>)> for Audio {
    fn from((audio, settings): (AnyAudio, Option<&Settings>)) -> Self {
        let mut audio: Audio = audio.into();
        if let Some(settings) = settings {
            audio = audio.merge_settings(settings.clone());
        }
        audio
    }
}

impl From<(PathBuf, Option<&Settings>)> for Audio {
    fn from(value: (PathBuf, Option<&Settings>)) -> Self {
        Self {
            file: value.0,
            settings: value.1.cloned(),
        }
    }
}

/// Convert file paths into audios.
pub fn paths_into_audios<K: PartialEq + Eq + Hash>(
    value: HashMap<K, PathBuf>,
    settings: Option<Settings>,
) -> HashMap<K, Audio> {
    value
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                Audio {
                    file: v,
                    settings: settings.clone(),
                },
            )
        })
        .collect()
}

/// Convert any audio into audios,
/// merging settings in the process.
pub fn any_audios_into_audios<K: PartialEq + Eq + Hash>(
    value: HashMap<K, AnyAudio>,
    settings: Option<Settings>,
) -> HashMap<K, Audio> {
    value
        .into_iter()
        .map(|(k, v)| {
            let mut v: Audio = v.into();
            if let Some(ref settings) = settings {
                v = v.merge_settings(settings.clone());
            }
            (k, v)
        })
        .collect()
}

impl Audio {
    /// Merge nested and parent settings.
    #[must_use]
    pub fn merge_settings(mut self, parent: Settings) -> Self {
        match &mut self.settings {
            Some(me) => {
                if me.start_time.is_none() && parent.start_time.is_some() {
                    me.start_time = parent.start_time;
                }
                if me.start_position.is_none() && parent.start_position.is_some() {
                    me.start_position = parent.start_position;
                }
                if me.volume.is_none() && parent.volume.is_some() {
                    me.volume = parent.volume;
                }
                if me.panning.is_none() && parent.panning.is_some() {
                    me.panning = parent.panning;
                }
                if me.r#loop.is_none() && parent.r#loop.is_some() {
                    me.r#loop = parent.r#loop;
                }
                if me.region.is_none() && parent.region.is_some() {
                    me.region = parent.region;
                }
                if me.playback_rate.is_none() && parent.playback_rate.is_some() {
                    me.playback_rate = parent.playback_rate;
                }
                if me.fade_in_tween.is_none() && parent.fade_in_tween.is_some() {
                    me.fade_in_tween = parent.fade_in_tween;
                }
            }
            None => {
                self.settings = Some(parent);
            }
        };
        self
    }
}

/// Describes usage made of audio.
///
/// Read more [in the book](https://cyb3rpsych0s1s.github.io/audioware/SETTINGS.html#-usage).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Usage {
    /// Audio played on-demand.
    OnDemand,
    /// Audio loaded all at once in-memory.
    InMemory,
    /// Audio streamed on-demand.
    Streaming,
}

/// Subtitle for audio.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Subtitle {
    Inline(String),
    Nested(DialogLine),
}

/// Dialog line.
#[derive(Debug, Clone, Deserialize)]
pub struct DialogLine {
    pub msg: String,
    pub line: ScnDialogLineType,
}

/// Manifest sources.
///
/// Also called ["sections" in the book](https://cyb3rpsych0s1s.github.io/audioware/SECTIONS.html).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    Sfx,
    Ono,
    Voices,
    Playlist,
    Music,
    Jingle,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Sfx => "sfx",
                Self::Ono => "onos",
                Self::Voices => "voices",
                Self::Playlist => "playlist",
                Self::Music => "music",
                Self::Jingle => "jingles",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use test_case::test_case;

    use super::Subtitle;

    #[test_case(r##"subtitle: "hello world""## ; "implicit subtitle")]
    #[test_case(r##"subtitle:
    msg: "hello world"
    line: radio"## ; "explicit subtitle")]
    fn subtitle(yaml: &str) {
        let subtitle = serde_yaml::from_str::<HashMap<String, Subtitle>>(yaml);
        dbg!("{}", &subtitle);
        assert!(subtitle.is_ok());
    }
}
