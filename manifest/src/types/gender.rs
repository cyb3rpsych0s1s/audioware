//! Used with [Codeware Localization](https://github.com/psiberx/cp2077-codeware/wiki#localization).

use std::fmt;

use serde::Deserialize;

/// See [Codeware](https://github.com/psiberx/cp2077-codeware/blob/main/scripts/Localization/Data/PlayerGender.reds).
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
                PlayerGender::Female => "Female",
                PlayerGender::Male => "Male",
            }
        )
    }
}

#[cfg(not(test))]
unsafe impl red4ext_rs::NativeRepr for PlayerGender {
    const NAME: &'static str = "Codeware.Localization.PlayerGender";
}

#[cfg(not(test))]
impl TryFrom<red4ext_rs::types::CName> for PlayerGender {
    type Error = crate::error::ConversionError;

    fn try_from(value: red4ext_rs::types::CName) -> Result<Self, Self::Error> {
        match value.as_str().to_lowercase().as_str() {
            "fem" | "female" => Ok(Self::Female),
            "male" => Ok(Self::Male),
            v => Err(crate::error::ConversionError::InvalidGender {
                value: v.to_string(),
            }),
        }
    }
}
