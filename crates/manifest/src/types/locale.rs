//! Used with [Codeware Localization](https://github.com/psiberx/cp2077-codeware/wiki#localization).

use fixed_map::Key;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use core::fmt;

/// Locale currently used for e.g. subtitles and UI texts.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

impl From<Locale> for SpokenLocale {
    fn from(value: Locale) -> Self {
        Self(value)
    }
}

#[cfg(not(test))]
impl TryFrom<red4ext_rs::types::CName> for SpokenLocale {
    type Error = crate::error::ConversionError;
    fn try_from(value: red4ext_rs::types::CName) -> Result<Self, Self::Error> {
        Ok(Self(Locale::try_from(value)?))
    }
}

/// Locale currently set for e.g. voices and dialogs.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WrittenLocale(Locale);

impl WrittenLocale {
    pub fn into_inner(self) -> Locale {
        self.0
    }
}

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

impl From<Locale> for WrittenLocale {
    fn from(value: Locale) -> Self {
        Self(value)
    }
}

#[cfg(not(test))]
impl TryFrom<red4ext_rs::types::CName> for WrittenLocale {
    type Error = crate::error::ConversionError;
    fn try_from(value: red4ext_rs::types::CName) -> Result<Self, Self::Error> {
        Ok(Self(Locale::try_from(value)?))
    }
}

/// Extended locale which supports both [Codeware locales][Locale] and [scnDialogLineLanguage][ScnDialogLineLanguage].
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Key,
    strum_macros::Display,
    strum_macros::EnumIter,
)]
#[repr(i64)]
pub enum LocaleExt {
    #[default]
    #[serde(rename = "en-us")]
    English = 0,
    #[serde(rename = "ht-ht")]
    Creole = 1,
    #[serde(rename = "jp-jp")]
    Japanese = 2,
    #[serde(rename = "ar-ar")]
    Arabic = 3,
    #[serde(rename = "ru-ru")]
    Russian = 4,
    #[serde(rename = "zh-cn")]
    SimplifiedChinese = 5,
    #[serde(rename = "pt-br")]
    BrazilianPortuguese = 6,
    #[serde(rename = "sw-ke")]
    Swahili = 7,
    #[serde(rename = "fr-fr")]
    French = 8,
    #[serde(rename = "pl-pl")]
    Polish = 9,
    #[serde(rename = "es-es")]
    Spanish = 10,
    #[serde(rename = "it-it")]
    Italian = 11,
    #[serde(rename = "de-de")]
    German = 12,
    #[serde(rename = "es-mx")]
    LatinAmericanSpanish = 13,
    #[serde(rename = "kr-kr")]
    Korean = 14,
    #[serde(rename = "zh-tw")]
    TraditionalChinese = 15,
    #[serde(rename = "cz-cz")]
    Czech = 16,
    #[serde(rename = "hu-hu")]
    Hungarian = 17,
    #[serde(rename = "tr-tr")]
    Turkish = 18,
    #[serde(rename = "th-th")]
    Thai = 19,
}

#[cfg(not(test))]
unsafe impl red4ext_rs::NativeRepr for LocaleExt {
    const NAME: &'static str = "Audioware.LocaleExt";
}

impl From<Locale> for LocaleExt {
    fn from(value: Locale) -> Self {
        match value {
            Locale::Polish => Self::Polish,
            Locale::English => Self::English,
            Locale::Spanish => Self::Spanish,
            Locale::French => Self::French,
            Locale::Italian => Self::Italian,
            Locale::German => Self::German,
            Locale::LatinAmericanSpanish => Self::LatinAmericanSpanish,
            Locale::Korean => Self::Korean,
            Locale::SimplifiedChinese => Self::SimplifiedChinese,
            Locale::Russian => Self::Russian,
            Locale::BrazilianPortuguese => Self::BrazilianPortuguese,
            Locale::Japanese => Self::Japanese,
            Locale::TraditionalChinese => Self::TraditionalChinese,
            Locale::Arabic => Self::Arabic,
            Locale::Czech => Self::Czech,
            Locale::Hungarian => Self::Hungarian,
            Locale::Turkish => Self::Turkish,
            Locale::Thai => Self::Thai,
        }
    }
}

impl From<ScnDialogLineLanguage> for LocaleExt {
    fn from(value: ScnDialogLineLanguage) -> Self {
        match value {
            ScnDialogLineLanguage::Origin => Self::English,
            ScnDialogLineLanguage::Creole => Self::Creole,
            ScnDialogLineLanguage::Japanese => Self::Japanese,
            ScnDialogLineLanguage::Arabic => Self::Arabic,
            ScnDialogLineLanguage::Russian => Self::Russian,
            ScnDialogLineLanguage::Chinese => Self::SimplifiedChinese,
            ScnDialogLineLanguage::Brasilian => Self::BrazilianPortuguese,
            ScnDialogLineLanguage::Swahili => Self::Swahili,
            ScnDialogLineLanguage::French => Self::French,
            ScnDialogLineLanguage::Polish => Self::Polish,
        }
    }
}

impl TryFrom<LocaleExt> for SpokenLocale {
    type Error = <Locale as TryFrom<LocaleExt>>::Error;

    fn try_from(value: LocaleExt) -> Result<Self, Self::Error> {
        Ok(Self(Locale::try_from(value)?))
    }
}

impl TryFrom<LocaleExt> for Locale {
    type Error = crate::error::ConversionError;

    fn try_from(value: LocaleExt) -> Result<Self, Self::Error> {
        match value {
            LocaleExt::Polish => Ok(Self::Polish),
            LocaleExt::English => Ok(Self::English),
            LocaleExt::Spanish => Ok(Self::Spanish),
            LocaleExt::French => Ok(Self::French),
            LocaleExt::Italian => Ok(Self::Italian),
            LocaleExt::German => Ok(Self::German),
            LocaleExt::LatinAmericanSpanish => Ok(Self::LatinAmericanSpanish),
            LocaleExt::Korean => Ok(Self::Korean),
            LocaleExt::SimplifiedChinese => Ok(Self::SimplifiedChinese),
            LocaleExt::Russian => Ok(Self::Russian),
            LocaleExt::BrazilianPortuguese => Ok(Self::BrazilianPortuguese),
            LocaleExt::Japanese => Ok(Self::Japanese),
            LocaleExt::TraditionalChinese => Ok(Self::TraditionalChinese),
            LocaleExt::Arabic => Ok(Self::Arabic),
            LocaleExt::Czech => Ok(Self::Czech),
            LocaleExt::Hungarian => Ok(Self::Hungarian),
            LocaleExt::Turkish => Ok(Self::Turkish),
            LocaleExt::Thai => Ok(Self::Thai),
            _ => Err(crate::error::ConversionError::UnsupportedLocale {
                r#type: "Codeware locale".to_string(),
                value: value.to_string(),
            }),
        }
    }
}

#[cfg(not(test))]
impl TryFrom<LocaleExt> for ScnDialogLineLanguage {
    type Error = crate::error::ConversionError;

    fn try_from(value: LocaleExt) -> Result<Self, Self::Error> {
        use red4ext_rs::NativeRepr;
        match value {
            LocaleExt::Polish => Ok(Self::Polish),
            LocaleExt::English => Ok(Self::Origin),
            LocaleExt::French => Ok(Self::French),
            LocaleExt::SimplifiedChinese => Ok(Self::Chinese),
            LocaleExt::Russian => Ok(Self::Russian),
            LocaleExt::BrazilianPortuguese => Ok(Self::Brasilian),
            LocaleExt::Japanese => Ok(Self::Japanese),
            LocaleExt::TraditionalChinese => Ok(Self::Chinese),
            LocaleExt::Arabic => Ok(Self::Arabic),
            LocaleExt::Swahili => Ok(Self::Swahili),
            LocaleExt::Creole => Ok(Self::Creole),
            _ => Err(crate::error::ConversionError::UnsupportedLocale {
                r#type: Self::NAME.to_string(),
                value: value.to_string(),
            }),
        }
    }
}

impl TryFrom<u32> for LocaleExt {
    type Error = crate::error::ConversionError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        for variant in LocaleExt::iter() {
            if value == variant as u32 {
                return Ok(variant);
            }
        }
        Err(crate::error::ConversionError::InvalidLocale {
            value: value.to_string(),
        })
    }
}

#[cfg(not(test))]
impl TryFrom<red4ext_rs::types::CName> for LocaleExt {
    type Error = crate::error::ConversionError;

    fn try_from(value: red4ext_rs::types::CName) -> Result<Self, Self::Error> {
        if value == red4ext_rs::types::CName::undefined() {
            return Ok(Self::English);
        }
        match value.as_str() {
            "en-us" | "English" => Ok(Self::English),
            "ht-ht" | "Creole" => Ok(Self::Creole),
            "jp-jp" | "Japanese" => Ok(Self::Japanese),
            "ar-ar" | "Arabic" => Ok(Self::Arabic),
            "ru-ru" | "Russian" => Ok(Self::Russian),
            "zh-cn" | "SimplifiedChinese" => Ok(Self::SimplifiedChinese),
            "pt-br" | "BrazilianPortuguese" => Ok(Self::BrazilianPortuguese),
            "sw-ke" | "sw-tz" | "Swahili" => Ok(Self::Swahili),
            "fr-fr" | "French" => Ok(Self::French),
            "pl-pl" | "Polish" => Ok(Self::Polish),
            "es-es" | "Spanish" => Ok(Self::Spanish),
            "it-it" | "Italian" => Ok(Self::Italian),
            "de-de" | "German" => Ok(Self::German),
            "es-mx" | "LatinAmericanSpanish" => Ok(Self::LatinAmericanSpanish),
            "kr-kr" | "Korean" => Ok(Self::Korean),
            "zh-tw" | "TraditionalChinese" => Ok(Self::TraditionalChinese),
            "cz-cz" | "Czech" => Ok(Self::Czech),
            "hu-hu" | "Hungarian" => Ok(Self::Hungarian),
            "tr-tr" | "Turkish" => Ok(Self::Turkish),
            "th-th" | "Thai" => Ok(Self::Thai),
            v => Err(Self::Error::InvalidLocale {
                value: v.to_string(),
            }),
        }
    }
}

/// See [Codeware](https://github.com/psiberx/cp2077-codeware/blob/main/scripts/Localization/Module/ModLocalizationProvider.reds#L9-L27).
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
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

impl TryFrom<u32> for Locale {
    type Error = crate::error::ConversionError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::Polish as u32 => Ok(Self::Polish),
            x if x == Self::English as u32 => Ok(Self::English),
            x if x == Self::Spanish as u32 => Ok(Self::Spanish),
            x if x == Self::French as u32 => Ok(Self::French),
            x if x == Self::Italian as u32 => Ok(Self::Italian),
            x if x == Self::German as u32 => Ok(Self::German),
            x if x == Self::LatinAmericanSpanish as u32 => Ok(Self::LatinAmericanSpanish),
            x if x == Self::Korean as u32 => Ok(Self::Korean),
            x if x == Self::SimplifiedChinese as u32 => Ok(Self::SimplifiedChinese),
            x if x == Self::Russian as u32 => Ok(Self::Russian),
            x if x == Self::BrazilianPortuguese as u32 => Ok(Self::BrazilianPortuguese),
            x if x == Self::Japanese as u32 => Ok(Self::Japanese),
            x if x == Self::TraditionalChinese as u32 => Ok(Self::TraditionalChinese),
            x if x == Self::Arabic as u32 => Ok(Self::Arabic),
            x if x == Self::Czech as u32 => Ok(Self::Czech),
            x if x == Self::Hungarian as u32 => Ok(Self::Hungarian),
            x if x == Self::Turkish as u32 => Ok(Self::Turkish),
            x if x == Self::Thai as u32 => Ok(Self::Thai),
            _ => Err(Self::Error::InvalidLocale {
                value: value.to_string(),
            }),
        }
    }
}

impl TryFrom<u32> for SpokenLocale {
    type Error = crate::error::ConversionError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self(Locale::try_from(value)?))
    }
}

impl TryFrom<u32> for WrittenLocale {
    type Error = crate::error::ConversionError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self(Locale::try_from(value)?))
    }
}

impl From<Locale> for u32 {
    fn from(value: Locale) -> Self {
        value as u32
    }
}

impl From<SpokenLocale> for u32 {
    fn from(value: SpokenLocale) -> Self {
        value.0.into()
    }
}

impl From<WrittenLocale> for u32 {
    fn from(value: WrittenLocale) -> Self {
        value.0.into()
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

/// See [NativeDB](https://nativedb.red4ext.com/scnDialogLineLanguage).
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Key,
    strum_macros::Display,
    strum_macros::EnumIter,
)]
pub enum ScnDialogLineLanguage {
    #[default]
    #[serde(rename = "en-us")]
    Origin = 0,
    #[serde(rename = "ht-ht")]
    Creole = 1,
    #[serde(rename = "jp-jp")]
    Japanese = 2,
    #[serde(rename = "ar-ar")]
    Arabic = 3,
    #[serde(rename = "ru-ru")]
    Russian = 4,
    #[serde(rename = "zh-cn")]
    Chinese = 5,
    #[serde(rename = "pt-br")]
    Brasilian = 6,
    #[serde(rename = "sw-ke")]
    Swahili = 7,
    #[serde(rename = "fr-fr")]
    French = 8,
    #[serde(rename = "pl-pl")]
    Polish = 9,
}

#[cfg(not(test))]
unsafe impl red4ext_rs::NativeRepr for ScnDialogLineLanguage {
    const NAME: &'static str = "scnDialogLineLanguage";
}

#[cfg(not(test))]
impl TryFrom<red4ext_rs::types::CName> for ScnDialogLineLanguage {
    type Error = crate::error::ConversionError;

    fn try_from(value: red4ext_rs::types::CName) -> Result<Self, Self::Error> {
        if value == red4ext_rs::types::CName::undefined() {
            return Ok(Self::Origin);
        }
        match value.as_str() {
            "pl-pl" => Ok(Self::Polish),
            "en-us" => Ok(Self::Origin),
            "ht-ht" => Ok(Self::Creole),
            "jp-jp" => Ok(Self::Japanese),
            "ar-ar" => Ok(Self::Arabic),
            "ru-ru" => Ok(Self::Russian),
            "zh-cn" | "zh-tw" => Ok(Self::Chinese),
            "pt-br" => Ok(Self::Brasilian),
            "sw-ke" | "sw-tz" => Ok(Self::Swahili),
            "fr-fr" => Ok(Self::French),
            v => Err(Self::Error::InvalidLocale {
                value: v.to_string(),
            }),
        }
    }
}
