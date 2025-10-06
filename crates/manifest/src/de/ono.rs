//! Onomatopeia definitions.

use serde::Deserialize;

use super::{AnyAudio, Audio, GenderBased, Settings, Usage, any_audios_into_audios};

#[derive(Debug, Deserialize)]
pub struct Ono {
    #[serde(flatten)]
    genders: GenderBased<AnyAudio>,
    usage: Option<Usage>,
    settings: Option<Settings>,
}

impl From<Ono> for (Usage, GenderBased<Audio>) {
    fn from(value: Ono) -> Self {
        (
            value.usage.unwrap_or(Usage::InMemory),
            any_audios_into_audios(value.genders, value.settings),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use test_case::test_case;

    use super::Ono;

    #[test_case(r##"id:
    fem: ./somewhere/sfx.wav
    male: ./somewhere/else/sfx.wav"## ; "implicit on-demand ono")]
    #[test_case(r##"id:
    fem: ./somewhere/sfx.wav
    male: ./somewhere/else/sfx.wav
    usage: on-demand"## ; "explicit on-demand ono")]
    fn ono(yaml: &str) {
        let ono = serde_yaml::from_str::<HashMap<String, Ono>>(yaml);
        dbg!(&ono);
        assert!(ono.is_ok());
    }
}
