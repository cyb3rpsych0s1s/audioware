use fixed_map::Key;
use red4ext_rs::types::CName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, Key)]
pub enum Locale {
    #[serde(rename = "pl-pl")]
    Polish,
    #[serde(rename = "en-us")]
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

impl Into<CName> for Locale {
    fn into(self) -> CName {
        CName::new(match self {
            Self::Polish => "pl-pl",
            Self::English => "en-us",
            Self::Spanish => "es-es",
            Self::French => "fr-fr",
            Self::Italian => "it-it",
            Self::German => "de-de",
            Self::LatinAmericanSpanish => "es-mx",
            Self::Korean => "kr-kr",
            Self::SimplifiedChinese => "zh-cn",
            Self::Russian => "ru-ru",
            Self::BrazilianPortuguese => "pt-br",
            Self::Japanese => "jp-jp",
            Self::TraditionalChinese => "zh-tw",
            Self::Arabic => "ar-ar",
            Self::Czech => "cz-cz",
            Self::Hungarian => "hu-hu",
            Self::Turkish => "tr-tr",
            Self::Thai => "th-th",
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
