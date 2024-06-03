//! # Bank keys
//!
//! Each [`Id`](super::Id) contains inside a [`Key`] which has 4 variants.
//! These [`Key`]s uphold the guarantee provided by their parent [`Id`](super::Id).

use std::hash::Hash;

use audioware_sys::interop::{gender::PlayerGender, locale::Locale};
use red4ext_rs::types::CName;

/// Key which can be either `Unique`, `Locale`, `Gender`
/// or `Both` (`Locale` and `Gender` at the same time).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    /// e.g. sfx
    Unique(UniqueKey),
    /// e.g. ono
    Gender(GenderKey),
    /// e.g. unique dialog
    Locale(LocaleKey),
    /// e.g. dual dialog
    Both(BothKey),
}

impl Key {
    pub fn as_unique(&self) -> Option<&UniqueKey> {
        match self {
            Key::Unique(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_gender(&self) -> Option<&GenderKey> {
        match self {
            Key::Gender(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_locale(&self) -> Option<&LocaleKey> {
        match self {
            Key::Locale(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_both(&self) -> Option<&BothKey> {
        match self {
            Key::Both(x) => Some(x),
            _ => None,
        }
    }
}

impl AsRef<CName> for Key {
    fn as_ref(&self) -> &CName {
        match self {
            Key::Unique(key) => key.as_ref(),
            Key::Gender(key) => key.as_ref(),
            Key::Locale(key) => key.as_ref(),
            Key::Both(key) => key.as_ref(),
        }
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Unique(key) => write!(f, "unique :{}", key),
            Key::Gender(key) => write!(f, "gender :{}", key),
            Key::Locale(key) => write!(f, "locale :{}", key),
            #[rustfmt::skip]
            Key::Both(key) =>     write!(f, "both   :{}", key),
        }
    }
}

impl PartialEq<(&CName, &Locale, &PlayerGender)> for Key {
    fn eq(&self, other: &(&CName, &Locale, &PlayerGender)) -> bool {
        match self {
            Key::Unique(key) => key.eq(other),
            Key::Gender(key) => key.eq(other),
            Key::Locale(key) => key.eq(other),
            Key::Both(key) => key.eq(other),
        }
    }
}

/// Any audio solely defined by its inner [`CName`].
///
/// e.g. sfx
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniqueKey(pub CName);
impl AsRef<CName> for UniqueKey {
    fn as_ref(&self) -> &CName {
        &self.0
    }
}
impl std::fmt::Display for UniqueKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}
impl PartialEq<(&CName, &Locale, &PlayerGender)> for UniqueKey {
    fn eq(&self, other: &(&CName, &Locale, &PlayerGender)) -> bool {
        &self.0 == other.0
    }
}

/// Any audio defined by both a [`CName`] and a [`PlayerGender`].
///
/// e.g. ono
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenderKey(pub CName, pub PlayerGender);
impl AsRef<CName> for GenderKey {
    fn as_ref(&self) -> &CName {
        &self.0
    }
}
impl std::fmt::Display for GenderKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{}]", self.0, self.1)
    }
}
impl PartialEq<(&CName, &Locale, &PlayerGender)> for GenderKey {
    fn eq(&self, other: &(&CName, &Locale, &PlayerGender)) -> bool {
        &self.0 == other.0 && &self.1 == other.2
    }
}

/// Any audio defined by both a [`CName`] and a [`Locale`].
///
/// e.g. dialog from a unique NPC translated in many languages
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocaleKey(pub CName, pub Locale);
impl AsRef<CName> for LocaleKey {
    fn as_ref(&self) -> &CName {
        &self.0
    }
}
impl std::fmt::Display for LocaleKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{}]", self.0, self.1)
    }
}
impl PartialEq<(&CName, &Locale, &PlayerGender)> for LocaleKey {
    fn eq(&self, other: &(&CName, &Locale, &PlayerGender)) -> bool {
        &self.0 == other.0 && &self.1 == other.1
    }
}

/// Any audio defined all by a [`CName`], [`Locale`] and [`PlayerGender`].
///
/// e.g. dialog for V translated in many languages for each gender.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BothKey(pub CName, pub Locale, pub PlayerGender);
impl AsRef<CName> for BothKey {
    fn as_ref(&self) -> &CName {
        &self.0
    }
}
impl std::fmt::Display for BothKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{}:{}]", self.0, self.1, self.2)
    }
}
impl PartialEq<(&CName, &Locale, &PlayerGender)> for BothKey {
    fn eq(&self, other: &(&CName, &Locale, &PlayerGender)) -> bool {
        &self.0 == other.0 && &self.1 == other.1 && &self.2 == other.2
    }
}

impl Hash for UniqueKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        u64::from(self.0.clone()).hash(state);
    }
}

impl Hash for GenderKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        u64::from(self.0.clone()).hash(state);
        self.1.hash(state);
    }
}

impl Hash for LocaleKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        u64::from(self.0.clone()).hash(state);
        self.1.hash(state);
    }
}

impl Hash for BothKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        u64::from(self.0.clone()).hash(state);
        self.1.hash(state);
        self.2.hash(state);
    }
}
