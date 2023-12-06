use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub enum Gender {
    Any,
    Male,
    Female,
    Shemale,
}