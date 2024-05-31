use std::hash::Hash;

use red4ext_rs::conv::NativeRepr;
use serde::Deserialize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, strum_macros::Display)]
#[repr(i64)]
pub enum PlayerGender {
    #[default]
    #[serde(rename = "fem")]
    Female = 0,
    #[serde(rename = "male")]
    Male = 1,
}

/// additional precaution to avoid key collisions
impl Hash for PlayerGender {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Female => "PlayerGender::Female".hash(state),
            Self::Male => "PlayerGender::Male".hash(state),
        }
    }
}

unsafe impl NativeRepr for PlayerGender {
    const NAME: &'static str = "Codeware.Localization.PlayerGender";
}
