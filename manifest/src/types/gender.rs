use std::fmt;

use red4ext_rs::NativeRepr;
use serde::Deserialize;

#[derive(Debug, Default, Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
pub enum PlayerGender {
    #[default]
    #[serde(rename = "fem")]
    Female = 0,
    #[serde(rename = "male")]
    Male = 1,
}

impl fmt::Display for PlayerGender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PlayerGender::Female => "female",
                PlayerGender::Male => "male",
            }
        )
    }
}

unsafe impl NativeRepr for PlayerGender {
    const NAME: &'static str = "Codeware.Localization.PlayerGender";
}
