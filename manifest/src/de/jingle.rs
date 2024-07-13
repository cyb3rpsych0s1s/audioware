use std::path::PathBuf;

use red4ext_rs_bindings::ScnDialogLineType;
use serde::Deserialize;

use super::Settings;
use crate::deserialize_optional_scn_dialog_line_type;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Jingle {
    Inline(PathBuf),
    Nested {
        file: PathBuf,
        captions: Vec<Caption>,
        #[serde(
            default,
            deserialize_with = "deserialize_optional_scn_dialog_line_type"
        )]
        line: Option<ScnDialogLineType>,
        settings: Option<Settings>,
    },
}

#[derive(Debug, Deserialize)]
pub struct Caption {
    pub starts: f32,
    pub msg: String,
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
