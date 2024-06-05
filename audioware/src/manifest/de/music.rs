use std::path::PathBuf;

use serde::Deserialize;

use super::Settings;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Music {
    Inline(PathBuf),
    Multi {
        file: PathBuf,
        settings: Option<Settings>,
    },
}

impl From<Music> for (PathBuf, Option<Settings>) {
    fn from(value: Music) -> Self {
        match value {
            Music::Inline(file) => (file, None),
            Music::Multi { file, settings } => (file, settings),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use test_case::test_case;

    use crate::manifest::de::Music;

    #[test_case(r##"new_intro: ./somewhere/music.wav"## ; "simple music")]
    fn music(yaml: &str) {
        let music = serde_yaml::from_str::<HashMap<String, Music>>(yaml);
        dbg!("{}", &music);
        assert!(music.is_ok());
    }
}
