use std::{collections::HashMap, path::PathBuf};

use audioware_sys::interop::{audio::ScnDialogLineType, gender::PlayerGender, locale::Locale};
use semver::Version;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub version: Version,
    pub sfx: Option<HashMap<String, Sfx>>,
    pub onos: Option<HashMap<String, Ono>>,
    pub voices: Option<HashMap<String, Voice>>,
    pub music: Option<HashMap<String, Music>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Sfx {
    Inline(PathBuf),
    Multi {
        #[serde(flatten)]
        props: Base,
    },
}

#[derive(Debug, Deserialize)]
pub struct Ono {
    #[serde(flatten)]
    pub genders: HashMap<PlayerGender, PathBuf>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    SingleInline {
        #[serde(flatten)]
        dialogs: HashMap<Locale, PathBuf>,
        usage: Option<Usage>,
    },
    SingleMulti {
        #[serde(flatten)]
        dialogs: HashMap<Locale, Dialog>,
        usage: Option<Usage>,
        line: Option<ScnDialogLineType>,
    },
    DualInline {
        #[serde(flatten)]
        dialogs: HashMap<Locale, HashMap<PlayerGender, PathBuf>>,
        usage: Option<Usage>,
    },
    DualMulti {
        #[serde(flatten)]
        dialogs: HashMap<Locale, Dialogs>,
        usage: Option<Usage>,
        line: Option<ScnDialogLineType>,
    },
}

#[derive(Debug, Deserialize)]
pub struct Playlist {
    pub name: String,
    pub songs: HashMap<String, PathBuf>,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Music(pub PathBuf);

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Jingle {
    Inline(PathBuf),
    Multi {
        file: PathBuf,
        captions: Vec<Caption>,
        line: Option<ScnDialogLineType>,
    },
}

#[derive(Debug, Deserialize)]
pub struct Base {
    pub file: PathBuf,
    pub usage: Usage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Usage {
    OnDemand,
    InMemory,
    Streaming,
}

#[derive(Debug, Deserialize)]
pub struct Dialog {
    pub file: PathBuf,
    pub subtitle: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Dialogs {
    Different {
        #[serde(flatten)]
        dialogs: HashMap<PlayerGender, Dialog>,
    },
    Shared {
        #[serde(flatten)]
        paths: HashMap<PlayerGender, PathBuf>,
        subtitle: String,
    },
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

#[derive(Debug, Deserialize)]
pub struct Caption {
    pub starts: f32,
    pub msg: String,
}

impl From<&Sfx> for Usage {
    fn from(value: &Sfx) -> Self {
        match value {
            Sfx::Inline(_) => Usage::InMemory,
            Sfx::Multi {
                props: Base { usage, .. },
            } => *usage,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use test_case::test_case;

    use crate::manifest::de::{Jingle, Music, Ono, Playlist, Sfx, Subtitle};

    #[test_case(r##"id: ./somewhere/sfx.wav"## ; "implicit on-demand sfx")]
    #[test_case(r##"id:
    file: ./somewhere/sfx.wav
    usage: on-demand"## ; "explicit on-demand sfx")]
    #[test_case(r##"id:
    file: ./somewhere/sfx.wav
    usage: in-memory"## ; "explicit in-memory sfx")]
    fn sfx(yaml: &str) {
        let sfx = serde_yaml::from_str::<HashMap<String, Sfx>>(yaml);
        dbg!("{}", &sfx);
        assert!(sfx.is_ok());
    }

    #[test_case(r##"id:
    fem: ./somewhere/sfx.wav
    male: ./somewhere/else/sfx.wav"## ; "implicit on-demand ono")]
    #[test_case(r##"id:
    fem: ./somewhere/sfx.wav
    male: ./somewhere/else/sfx.wav
    usage: on-demand"## ; "explicit on-demand ono")]
    fn ono(yaml: &str) {
        let ono = serde_yaml::from_str::<HashMap<String, Ono>>(yaml);
        dbg!("{}", &ono);
        assert!(ono.is_ok());
    }

    #[test_case(r##"subtitle: "hello world""## ; "implicit subtitle")]
    #[test_case(r##"subtitle:
    msg: "hello world"
    line: radio"## ; "explicit subtitle")]
    fn subtitle(yaml: &str) {
        let subtitle = serde_yaml::from_str::<HashMap<String, Subtitle>>(yaml);
        dbg!("{}", &subtitle);
        assert!(subtitle.is_ok());
    }

    mod unique_dialog {
        use crate::manifest::de::Voice;
        use std::collections::HashMap;
        use test_case::test_case;

        #[test_case(r##"id:
        en-us: ./somewhere/sfx.wav"## ; "implicit on-demand unique dialog no subtitle")]
        #[test_case(r##"id:
        en-us:
            file: ./somewhere/sfx.wav
            subtitle: "hello world""## ; "implicit on-demand unique dialog with subtitle")]
        #[test_case(r##"id:
        en-us:
            file: ./somewhere/sfx.wav
            subtitle: "hello world"
        line: radio"## ; "implicit on-demand unique dialog with subtitle and line type")]
        #[test_case(r##"id:
        en-us: ./somewhere/sfx.wav
        fr-fr: ./somewhere/sfx.wav"## ; "format must be consistent across locales when there's no subtitle")]
        #[test_case(r##"id:
        en-us:
            file: ./somewhere/sfx.wav
            subtitle: "hello world"
        fr-fr:
            file: ./somewhere/else/sfx.wav
            subtitle: "bonjour tout le monde"
        line: radio"## ; "format must be consistent across locales when there are subtitles")]
        fn basic_format(yaml: &str) {
            let unique_dialog = serde_yaml::from_str::<HashMap<String, Voice>>(yaml);
            dbg!("{}", &unique_dialog);
            assert!(unique_dialog.is_ok());
        }

        #[test_case(r##"id:
        en-us:
            file: ./somewhere/sfx.wav
            subtitle: "hello world"
        fr-fr: ./somewhere/else/sfx.wav"## ; "format must be consistent")]
        fn incompatibility(yaml: &str) {
            let unique_dialog = serde_yaml::from_str::<HashMap<String, Voice>>(yaml);
            dbg!("{}", &unique_dialog);
            assert!(unique_dialog.is_err());
        }
    }

    mod dual_dialog {
        use std::collections::HashMap;

        use crate::manifest::de::Voice;
        use test_case::test_case;

        #[test_case(r##"id:
        en-us:
            fem: ./somewhere/sfx.wav
            male: ./somewhere/else/sfx.wav
        fr-fr:
            fem: ./elsewhere/sfx.wav
            male: ./elsewhere/else/sfx.wav"## ; "dual dialog without subtitle")]
        fn basic_format_without_subtitle(yaml: &str) {
            let dual_dialog = serde_yaml::from_str::<HashMap<String, Voice>>(yaml);
            dbg!("{}", &dual_dialog);
            assert!(dual_dialog.is_ok());
        }

        #[test_case(r##"id:
        en-us:
            fem: ./somewhere/sfx.wav
            male: ./somewhere/else/sfx.wav
            subtitle: "hello world"
        line: radio"## ; "dual dialog with shared subtitle")]
        #[test_case(r##"id:
        en-us:
            fem:
                file: ./somewhere/sfx.wav
                subtitle: "hello world"
            male:
                file: ./somewhere/else/sfx.wav
                subtitle: "hello world""## ; "dual dialog with different subtitles and default line")]
        fn basic_format_with_subtitles(yaml: &str) {
            let dual_dialog = serde_yaml::from_str::<HashMap<String, Voice>>(yaml);
            dbg!("{}", &dual_dialog);
            assert!(dual_dialog.is_ok());
        }

        #[test_case(r##"id:
        en-us:
            fem: ./somewhere/sfx.wav
            male: ./somewhere/else/sfx.wav
            subtitle: "hello world"
        fr-fr:
            fem: ./somewhere/sfx.wav
            male: ./somewhere/else/sfx.wav
        line: radio"## ; "format must be consistent, mixing sub/no-sub is not allowed")]
        #[test_case(r##"id:
        en-us:
            fem: ./somewhere/sfx.wav
            male: ./somewhere/else/sfx.wav
        fr-fr:
            fem: ./somewhere/sfx.wav
            male: ./somewhere/else/sfx.wav
        line: radio"## ; "format must be consistent, if there's no subtitle there shouldn't be any line")]
        fn incompatibility(yaml: &str) {
            let dual_dialog = serde_yaml::from_str::<HashMap<String, Voice>>(yaml);
            dbg!("{}", &dual_dialog);
            assert!(dual_dialog.is_err());
        }
    }

    #[test_case(r##"new_intro: ./somewhere/music.wav"## ; "simple music")]
    fn music(yaml: &str) {
        let playlist = serde_yaml::from_str::<HashMap<String, Music>>(yaml);
        dbg!("{}", &playlist);
        assert!(playlist.is_ok());
    }

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
