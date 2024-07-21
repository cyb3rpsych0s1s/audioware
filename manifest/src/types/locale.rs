use fixed_map::Key;
use serde::{Deserialize, Serialize};

use core::fmt;

/// locale currently used for e.g. subtitles and UI texts.
#[derive(Debug, Default, Clone, Copy)]
pub struct SpokenLocale(Locale);

impl fmt::Display for SpokenLocale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<SpokenLocale> for Locale {
    fn eq(&self, other: &SpokenLocale) -> bool {
        other.0.eq(self)
    }
}

impl PartialEq<Locale> for SpokenLocale {
    fn eq(&self, other: &Locale) -> bool {
        self.0.eq(other)
    }
}

#[cfg(not(test))]
impl TryFrom<red4ext_rs::types::CName> for SpokenLocale {
    type Error = crate::ConversionError;
    fn try_from(value: red4ext_rs::types::CName) -> Result<Self, Self::Error> {
        Ok(Self(Locale::try_from(value)?))
    }
}

/// locale currently set for e.g. voices and dialogs.
#[derive(Debug, Default, Clone, Copy)]
pub struct WrittenLocale(Locale);

impl fmt::Display for WrittenLocale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<Locale> for WrittenLocale {
    fn eq(&self, other: &Locale) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<WrittenLocale> for Locale {
    fn eq(&self, other: &WrittenLocale) -> bool {
        other.0.eq(self)
    }
}

#[cfg(not(test))]
impl TryFrom<red4ext_rs::types::CName> for WrittenLocale {
    type Error = crate::ConversionError;
    fn try_from(value: red4ext_rs::types::CName) -> Result<Self, Self::Error> {
        Ok(Self(Locale::try_from(value)?))
    }
}

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

#[cfg(not(test))]
impl From<Locale> for red4ext_rs::types::CName {
    fn from(val: Locale) -> Self {
        red4ext_rs::types::CName::new(match val {
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

#[cfg(not(test))]
impl TryFrom<red4ext_rs::types::CName> for Locale {
    type Error = crate::error::ConversionError;

    fn try_from(value: red4ext_rs::types::CName) -> Result<Self, Self::Error> {
        if value == red4ext_rs::types::CName::undefined() {
            return Ok(Self::English);
        }
        match value.as_str() {
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
            v => Err(Self::Error::InvalidLocale {
                value: v.to_string(),
            }),
        }
    }
}
