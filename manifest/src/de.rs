//! Manifest definitions.

use std::{collections::HashMap, fmt, hash::Hash, path::PathBuf};

use crate::ScnDialogLineType;
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

/// [`Audio`] with optional [`Usage`].
#[derive(Debug, Deserialize)]
pub struct UsableAudio {
    #[serde(flatten)]
    pub audio: Audio,
    pub usage: Option<Usage>,
}

/// Audio file path with optional [`Settings`].
#[derive(Debug, Deserialize, Clone)]
pub struct Audio {
    pub file: PathBuf,
    pub settings: Option<Settings>,
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
    fn from(value: (AnyAudio, Option<&Settings>)) -> Self {
        let mut audio: Audio = value.0.into();
        if let Some(settings) = value.1 {
            audio.merge_settings(settings.clone());
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

pub fn any_audios_into_audios<K: PartialEq + Eq + Hash>(
    value: HashMap<K, AnyAudio>,
    settings: Option<Settings>,
) -> HashMap<K, Audio> {
    value
        .into_iter()
        .map(|(k, v)| {
            let mut v: Audio = v.into();
            if let Some(ref settings) = settings {
                v.merge_settings(settings.clone());
            }
            (k, v)
        })
        .collect()
}

impl Audio {
    /// Merge nested and parent settings.
    pub fn merge_settings(&mut self, parent: Settings) {
        match &mut self.settings {
            Some(me) => {
                if me.start_time.is_none() && parent.start_time.is_some() {
                    me.start_time = parent.start_time;
                }
                if me.volume.is_none() && parent.volume.is_some() {
                    me.volume = parent.volume;
                }
            }
            None => {
                self.settings = Some(parent);
            }
        };
    }
}

/// Describes usage made of audio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Usage {
    /// audio played on-demand.
    OnDemand,
    /// audio loaded all at once in-memory.
    InMemory,
    /// audio streamed on-demand.
    Streaming,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Subtitle {
    Inline(String),
    Nested(DialogLine),
}

#[derive(Debug, Clone, Deserialize)]
pub struct DialogLine {
    pub msg: String,
    pub line: ScnDialogLineType,
}

/// Manifest sources.
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
