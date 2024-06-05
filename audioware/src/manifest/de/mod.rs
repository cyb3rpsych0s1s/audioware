use std::{collections::HashMap, hash::Hash, path::PathBuf};

use audioware_sys::interop::audio::ScnDialogLineType;
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

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub version: Version,
    pub sfx: Option<HashMap<String, Sfx>>,
    pub onos: Option<HashMap<String, Ono>>,
    pub voices: Option<HashMap<String, Voice>>,
    pub music: Option<HashMap<String, Music>>,
}

#[derive(Debug, Deserialize)]
pub struct UsableAudio {
    #[serde(flatten)]
    pub audio: Audio,
    pub usage: Usage,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Audio {
    pub file: PathBuf,
    pub settings: Option<Settings>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum AnyAudio {
    Inline(PathBuf),
    Multi(Audio),
}

impl From<AnyAudio> for Audio {
    fn from(value: AnyAudio) -> Self {
        match value {
            AnyAudio::Inline(file) => Self {
                file,
                settings: None,
            },
            AnyAudio::Multi(audio) => audio,
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
        let mut audio: Audio = Audio {
            file: value.0,
            settings: None,
        };
        if let Some(settings) = value.1 {
            audio.merge_settings(settings.clone());
        }
        audio
    }
}

pub fn paths_into_audios<K: PartialEq + Eq + Hash>(
    value: HashMap<K, PathBuf>,
    settings: Option<Settings>,
) -> HashMap<K, Audio> {
    value
        .into_iter()
        .map(|(k, v)| {
            let mut v: Audio = Audio {
                file: v,
                settings: None,
            };
            if let Some(ref settings) = settings {
                v.merge_settings(settings.clone());
            }
            (k, v)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Usage {
    OnDemand,
    InMemory,
    Streaming,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Subtitle {
    Inline(String),
    Multi(DialogLine),
}

#[derive(Debug, Clone, Deserialize)]
pub struct DialogLine {
    pub msg: String,
    pub line: ScnDialogLineType,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use test_case::test_case;

    use crate::manifest::de::Subtitle;

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
