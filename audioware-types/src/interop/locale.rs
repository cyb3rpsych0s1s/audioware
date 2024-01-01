use fixed_map::Key;
use red4ext_rs::types::CName;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    Hash,
    Key,
    strum_macros::Display,
    strum_macros::EnumIter,
)]
pub enum Locale {
    #[serde(rename = "pl-pl")]
    Polish,
    #[serde(rename = "en-us")]
    #[default]
    English,
    #[serde(rename = "es-es")]
    Spanish,
    #[serde(rename = "fr-fr")]
    French,
    #[serde(rename = "it-it")]
    Italian,
    #[serde(rename = "de-de")]
    German,
    #[serde(rename = "es-mx")]
    LatinAmericanSpanish,
    #[serde(rename = "kr-kr")]
    Korean,
    #[serde(rename = "zh-cn")]
    SimplifiedChinese,
    #[serde(rename = "ru-ru")]
    Russian,
    #[serde(rename = "pt-br")]
    BrazilianPortuguese,
    #[serde(rename = "jp-jp")]
    Japanese,
    #[serde(rename = "zh-tw")]
    TraditionalChinese,
    #[serde(rename = "ar-ar")]
    Arabic,
    #[serde(rename = "cz-cz")]
    Czech,
    #[serde(rename = "hu-hu")]
    Hungarian,
    #[serde(rename = "tr-tr")]
    Turkish,
    #[serde(rename = "th-th")]
    Thai,
}

impl From<Locale> for CName {
    fn from(val: Locale) -> Self {
        CName::new(match val {
            Locale::Polish => "pl-pl",
            Locale::English => "en-us",
            Locale::Spanish => "es-es",
            Locale::French => "fr-fr",
            Locale::Italian => "it-it",
            Locale::German => "de-de",
            Locale::LatinAmericanSpanish => "es-mx",
            Locale::Korean => "kr-kr",
            Locale::SimplifiedChinese => "zh-cn",
            Locale::Russian => "ru-ru",
            Locale::BrazilianPortuguese => "pt-br",
            Locale::Japanese => "jp-jp",
            Locale::TraditionalChinese => "zh-tw",
            Locale::Arabic => "ar-ar",
            Locale::Czech => "cz-cz",
            Locale::Hungarian => "hu-hu",
            Locale::Turkish => "tr-tr",
            Locale::Thai => "th-th",
        })
    }
}

impl TryFrom<CName> for Locale {
    type Error = anyhow::Error;

    fn try_from(value: CName) -> Result<Self, Self::Error> {
        match red4ext_rs::ffi::resolve_cname(&value) {
            "pl-pl" => Ok(Self::Polish),
            "en-us" => Ok(Self::English),
            "es-es" => Ok(Self::Spanish),
            "fr-fr" => Ok(Self::French),
            "it-it" => Ok(Self::Italian),
            "de-de" => Ok(Self::German),
            "es-mx" => Ok(Self::LatinAmericanSpanish),
            "kr-kr" => Ok(Self::Korean),
            "zh-cn" => Ok(Self::SimplifiedChinese),
            "ru-ru" => Ok(Self::Russian),
            "pt-br" => Ok(Self::BrazilianPortuguese),
            "jp-jp" => Ok(Self::Japanese),
            "zh-tw" => Ok(Self::TraditionalChinese),
            "ar-ar" => Ok(Self::Arabic),
            "cz-cz" => Ok(Self::Czech),
            "hu-hu" => Ok(Self::Hungarian),
            "tr-tr" => Ok(Self::Turkish),
            "th-th" => Ok(Self::Thai),
            v => anyhow::bail!(format!("invalid Locale ({})", v)),
        }
    }
}
