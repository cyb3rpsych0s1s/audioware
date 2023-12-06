use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub enum Locale {
    #[serde(rename = "en-us")]
    English,
    #[serde(rename = "fr-fr")]
    French,
}
