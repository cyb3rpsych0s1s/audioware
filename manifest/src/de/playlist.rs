use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

use super::Settings;

#[derive(Debug, Deserialize)]
pub struct Playlist {
    pub name: String,
    pub songs: HashMap<String, Song>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Song {
    Inline(PathBuf),
    Nested {
        file: PathBuf,
        settings: Option<Settings>,
    },
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use test_case::test_case;

    use super::Playlist;

    #[test_case(r##"summer_chill:
    name: "Summer chill"
    songs:
        come_again: ./somewhere/song.wav
        everyday: ./somewhere/else/song.wav"## ; "simple playlist")]
    fn playlist(yaml: &str) {
        let playlist = serde_yaml::from_str::<HashMap<String, Playlist>>(yaml);
        dbg!("{}", &playlist);
        assert!(playlist.is_ok());
    }
}
