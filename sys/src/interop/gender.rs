use red4ext_rs::conv::NativeRepr;
use serde::Deserialize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deserialize, strum_macros::Display)]
#[repr(i64)]
pub enum PlayerGender {
    #[default]
    #[serde(rename = "fem")]
    Female = 0,
    #[serde(rename = "male")]
    Male = 1,
}

unsafe impl NativeRepr for PlayerGender {
    const NAME: &'static str = "Codeware.Localization.PlayerGender";
}
