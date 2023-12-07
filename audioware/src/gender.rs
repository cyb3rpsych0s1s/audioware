use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Gender {
    #[serde(rename = "fem")]
    Female,
    #[serde(rename = "male")]
    Male,
}
