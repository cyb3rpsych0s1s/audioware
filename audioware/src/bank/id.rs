use std::{hash::Hash, path::PathBuf};

use audioware_sys::interop::{gender::PlayerGender, locale::Locale};
use red4ext_rs::types::CName;

use super::{BothKey, GenderKey, Key, LocaleKey, UniqueKey};

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
