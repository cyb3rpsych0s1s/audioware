use std::{hash::Hash, path::PathBuf};

use audioware_sys::interop::{gender::PlayerGender, locale::Locale};
use red4ext_rs::types::CName;

/// special type whose audio data is guaranteed to both exist in banks and be valid
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Id {
    OnDemand(Usage),
    InMemory(Key),
}

impl Hash for Id {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let key: &Key = self.as_ref();
        key.hash(state);
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Id::OnDemand(usage) => write!(f, "|on-demand| {}", usage),
            Id::InMemory(key) => write!(f, "|in-memory| {}", key),
        }
    }
}

impl AsRef<CName> for Id {
    fn as_ref(&self) -> &CName {
        match self {
            Id::InMemory(key)
            | Id::OnDemand(Usage::Static(key, _))
            | Id::OnDemand(Usage::Streaming(key, _)) => match key {
                Key::Unique(UniqueKey(key))
                | Key::Gender(GenderKey(key, _))
                | Key::Locale(LocaleKey(key, _))
                | Key::Both(BothKey(key, ..)) => key,
            },
        }
    }
}

impl AsRef<Key> for Id {
    fn as_ref(&self) -> &Key {
        match self {
            Id::OnDemand(Usage::Static(key, _)) | Id::OnDemand(Usage::Streaming(key, _)) => key,
            Id::InMemory(key) => key,
        }
    }
}

impl PartialEq<(&CName, &Locale, &PlayerGender)> for Id {
    fn eq(&self, other: &(&CName, &Locale, &PlayerGender)) -> bool {
        match self {
            Id::OnDemand(usage) => usage.eq(other),
            Id::InMemory(key) => key.eq(other),
        }
    }
}

impl PartialEq<(&CName, &Locale, Option<&PlayerGender>)> for Id {
    fn eq(&self, other: &(&CName, &Locale, Option<&PlayerGender>)) -> bool {
        if let Some(gender) = other.2 {
            return self.eq(&(other.0, other.1, gender));
        }
        let key: &Key = self.as_ref();
        match key {
            Key::Unique(key) => key.as_ref() == other.0,
            Key::Locale(LocaleKey(key, locale)) => key == other.0 && locale == other.1,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Usage {
    Static(Key, PathBuf),
    Streaming(Key, PathBuf),
}

impl std::fmt::Display for Usage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Usage::Static(key, path) => write!(
                f,
                "static:{} ({})",
                key,
                path.display().to_string().as_str()
            ),
            Usage::Streaming(key, path) => write!(
                f,
                "streaming:{} ({})",
                key,
                path.display().to_string().as_str()
            ),
        }
    }
}

impl PartialEq<(&CName, &Locale, &PlayerGender)> for Usage {
    fn eq(&self, other: &(&CName, &Locale, &PlayerGender)) -> bool {
        match self {
            Usage::Static(key, _) => key.eq(other),
            Usage::Streaming(key, _) => key.eq(other),
        }
    }
}

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
