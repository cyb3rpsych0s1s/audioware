use std::path::PathBuf;

use crate::ScnDialogLineType;
use serde::Deserialize;

use super::{Audio, Settings};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Jingle {
    Inline(PathBuf),
    Nested {
        file: PathBuf,
        captions: Vec<Caption>,
        line: Option<ScnDialogLineType>,
        settings: Option<Settings>,
    },
}

impl Jingle {
    pub fn captions(&self) -> Option<&[Caption]> {
        match self {
            Jingle::Inline(_) => None,
            Jingle::Nested { captions, .. } => Some(captions.as_slice()),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Caption {
    pub starts: f32,
    pub msg: String,
}

impl From<&Jingle> for Audio {
    fn from(value: &Jingle) -> Self {
        match value {
            Jingle::Inline(file) => Self {
                file: file.clone(),
                settings: None,
            },
            Jingle::Nested {
                file,
                captions,
                line,
                settings,
            } => Self {
                file: file.clone(),
                settings: settings.clone(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use test_case::test_case;

    use super::Jingle;

    #[test_case(r##"new_jingle: ./somewhere/music.wav"## ; "simple jingle")]
    #[test_case(r##"subtitled_jingle:
    file: ./somewhere/music.wav
    captions:
        - starts: 1.6
          msg: "hello world""## ; "jingle with timed localized captions")]
    #[test_case(r##"elaborated_jingle:
    file: ./somewhere/music.wav
    captions:
        - starts: 1.6
          msg: "hello world"
        - starts: 6
          msg: "goodbye"
    line: radio"## ; "jingle with timed localized captions and specific line")]
    fn jingle(yaml: &str) {
        let jingle = serde_yaml::from_str::<HashMap<String, Jingle>>(yaml);
        dbg!("{}", &jingle);
        assert!(jingle.is_ok());
    }
}
