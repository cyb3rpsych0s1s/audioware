use core::fmt;
use std::ops::Deref;

use audioware_manifest::{ConversionError, Locale};
use red4ext_rs::types::CName;

/// locale currently used for e.g. subtitles and UI texts.
#[derive(Debug, Default, Clone, Copy)]
pub struct SpokenLocale(Locale);

/// locale currently set for e.g. voices and dialogs.
#[derive(Debug, Default, Clone, Copy)]
pub struct WrittenLocale(Locale);

impl Deref for SpokenLocale {
    type Target = Locale;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for WrittenLocale {
    type Target = Locale;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for SpokenLocale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for WrittenLocale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<CName> for SpokenLocale {
    type Error = ConversionError;
    fn try_from(value: CName) -> Result<Self, Self::Error> {
        Ok(Self(Locale::try_from(value)?))
    }
}

impl TryFrom<CName> for WrittenLocale {
    type Error = ConversionError;
    fn try_from(value: CName) -> Result<Self, Self::Error> {
        Ok(Self(Locale::try_from(value)?))
    }
}

impl PartialEq<Locale> for SpokenLocale {
    fn eq(&self, other: &Locale) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<Locale> for WrittenLocale {
    fn eq(&self, other: &Locale) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<SpokenLocale> for Locale {
    fn eq(&self, other: &SpokenLocale) -> bool {
        other.0.eq(self)
    }
}

impl PartialEq<WrittenLocale> for Locale {
    fn eq(&self, other: &WrittenLocale) -> bool {
        other.0.eq(self)
    }
}
