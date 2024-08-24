//! Music definitions.

use std::path::PathBuf;

use serde::Deserialize;

use super::{Audio, UsableAudio};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Music {
    Inline(PathBuf),
    Nested {
        #[serde(flatten)]
        props: UsableAudio,
    },
}

impl From<Music> for UsableAudio {
    fn from(value: Music) -> Self {
        match value {
            Music::Inline(file) => Self {
                audio: Audio {
                    file,
                    settings: None,
                },
                usage: None,
            },
            Music::Nested { props } => props,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use test_case::test_case;

    use super::Music;

    #[test_case(r##"new_intro: ./somewhere/music.wav"## ; "simple music")]
    fn music(yaml: &str) {
        let music = serde_yaml::from_str::<HashMap<String, Music>>(yaml);
        dbg!("{}", &music);
        assert!(music.is_ok());
    }
}
