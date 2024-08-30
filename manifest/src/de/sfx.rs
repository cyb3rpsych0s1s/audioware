//! SFX definitions.

use std::path::PathBuf;

use serde::Deserialize;

use super::{Audio, UsableAudio, Usage};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Sfx {
    Inline(PathBuf),
    Nested {
        #[serde(flatten)]
        props: UsableAudio,
    },
}

impl From<Sfx> for UsableAudio {
    fn from(value: Sfx) -> Self {
        match value {
            Sfx::Inline(file) => UsableAudio {
                audio: Audio {
                    file,
                    settings: None,
                },
                usage: Some(Usage::InMemory),
            },
            Sfx::Nested { props } => props,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use test_case::test_case;

    use super::Sfx;

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
}
