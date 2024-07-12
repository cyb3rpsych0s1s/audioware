use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
pub enum PlayerGender {
    #[serde(rename = "fem")]
    Female,
    #[serde(rename = "male")]
    Male,
}
