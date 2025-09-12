use audioware_manifest::{Locale, PlayerGender};
use red4ext_rs::types::Cruid;

use crate::error::registry::ErrorDisplay;

/// Key which can be either `Locale`
/// or `Both` (`Locale` and `Gender` at the same time).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SceneKey {
    /// e.g. unique scene dialog
    Locale(SceneLocaleKey),
    /// e.g. dual scene dialog
    Both(SceneBothKey),
}

impl SceneKey {
    pub fn as_locale(&self) -> Option<&SceneLocaleKey> {
        match self {
            Self::Locale(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_both(&self) -> Option<&SceneBothKey> {
        match self {
            Self::Both(x) => Some(x),
            _ => None,
        }
    }
    pub fn locale(&self) -> Locale {
        match self {
            Self::Both(SceneBothKey(_, locale, _)) | Self::Locale(SceneLocaleKey(_, locale)) => {
                *locale
            }
        }
    }
}

impl AsRef<Cruid> for SceneKey {
    fn as_ref(&self) -> &Cruid {
        match self {
            Self::Locale(key) => key.as_ref(),
            Self::Both(key) => key.as_ref(),
        }
    }
}

impl std::fmt::Display for SceneKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Locale(key) => write!(f, "locale :{key}"),
            #[rustfmt::skip]
            Self::Both(key) =>     write!(f, "both   :{key}"),
        }
    }
}

/// Any audio defined by both a [Cruid] and a [Locale].
///
/// e.g. scene dialog from a unique NPC translated in many languages
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SceneLocaleKey(pub Cruid, pub Locale);
impl AsRef<Cruid> for SceneLocaleKey {
    fn as_ref(&self) -> &Cruid {
        &self.0
    }
}
impl std::fmt::Display for SceneLocaleKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{}]", self.0.error_display(), self.1)
    }
}
impl PartialEq<(&Cruid, &Locale)> for SceneLocaleKey {
    fn eq(&self, other: &(&Cruid, &Locale)) -> bool {
        &self.0 == other.0 && &self.1 == other.1
    }
}
impl From<SceneLocaleKey> for SceneKey {
    fn from(value: SceneLocaleKey) -> Self {
        Self::Locale(value)
    }
}

/// Any audio defined all by a [Cruid], [Locale] and [PlayerGender].
///
/// e.g. scene dialog for V translated in many languages for each gender.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SceneBothKey(pub Cruid, pub Locale, pub PlayerGender);
impl AsRef<Cruid> for SceneBothKey {
    fn as_ref(&self) -> &Cruid {
        &self.0
    }
}
impl std::fmt::Display for SceneBothKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{}:{}]", self.0.error_display(), self.1, self.2)
    }
}
impl PartialEq<(&Cruid, &Locale, &PlayerGender)> for SceneBothKey {
    fn eq(&self, other: &(&Cruid, &Locale, &PlayerGender)) -> bool {
        &self.0 == other.0 && &self.1 == other.1 && &self.2 == other.2
    }
}
impl From<SceneBothKey> for SceneKey {
    fn from(value: SceneBothKey) -> Self {
        Self::Both(value)
    }
}
