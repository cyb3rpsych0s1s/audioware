use std::fmt;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
pub enum PlayerGender {
    #[serde(rename = "fem")]
    Female,
    #[serde(rename = "male")]
    Male,
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
