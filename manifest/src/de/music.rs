use serde::Deserialize;

use super::{AnyAudio, Audio};

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Music(AnyAudio);

impl From<Music> for Audio {
    fn from(value: Music) -> Self {
        value.0.into()
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
