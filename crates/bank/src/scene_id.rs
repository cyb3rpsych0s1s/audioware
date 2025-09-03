use std::path::PathBuf;

use audioware_manifest::SceneDialogs;
use red4ext_rs::types::Cruid;

use crate::SceneKey;

/// Special type whose audio data is guaranteed to both exist in banks and be valid.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SceneId {
    OnDemand(SceneUsage),
    InMemory(SceneKey),
}

impl std::fmt::Display for SceneId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SceneId::OnDemand(usage, ..) => write!(f, "|on-demand| {usage}"),
            SceneId::InMemory(key, ..) => write!(f, "|in-memory| {key}"),
        }
    }
}

impl AsRef<SceneKey> for SceneId {
    fn as_ref(&self) -> &SceneKey {
        match self {
            Self::OnDemand(scene_usage) => scene_usage.as_ref(),
            Self::InMemory(scene_key) => scene_key,
        }
    }
}

impl AsRef<Cruid> for SceneId {
    fn as_ref(&self) -> &Cruid {
        match self {
            SceneId::OnDemand(scene_usage) => scene_usage.as_ref(),
            SceneId::InMemory(scene_key) => scene_key.as_ref(),
        }
    }
}

impl PartialEq<SceneUsage> for SceneId {
    fn eq(&self, other: &SceneUsage) -> bool {
        match self {
            SceneId::OnDemand(scene_usage) => scene_usage == other,
            SceneId::InMemory(scene_key) => scene_key == AsRef::<SceneKey>::as_ref(other),
        }
    }
}

impl PartialEq<SceneUsage> for SceneKey {
    fn eq(&self, other: &SceneUsage) -> bool {
        match other {
            SceneUsage::Static(scene_key, ..) | SceneUsage::Streaming(scene_key, ..) => {
                scene_key == other
            }
        }
    }
}

impl PartialEq<SceneKey> for SceneId {
    fn eq(&self, other: &SceneKey) -> bool {
        match self {
            SceneId::OnDemand(scene_usage) => other == scene_usage,
            SceneId::InMemory(scene_key) => scene_key == other,
        }
    }
}

impl PartialEq<(&Cruid, &SceneDialogs)> for SceneUsage {
    fn eq(&self, other: &(&Cruid, &SceneDialogs)) -> bool {
        match self {
            SceneUsage::Static(scene_key, ..) | SceneUsage::Streaming(scene_key, ..) => {
                scene_key == other
            }
        }
    }
}

impl PartialEq<(&Cruid, &SceneDialogs)> for SceneKey {
    fn eq(&self, other: &(&Cruid, &SceneDialogs)) -> bool {
        match self {
            SceneKey::Locale(scene_locale_key) if *other.0 == scene_locale_key.0 => {
                match &other.1 {
                    SceneDialogs::SingleInline { dialogs, .. } => {
                        dialogs.contains_key(&scene_locale_key.1)
                    }
                    SceneDialogs::DualInline { dialogs, .. } => {
                        dialogs.contains_key(&scene_locale_key.1)
                    }
                }
            }
            SceneKey::Both(scene_both_key) if *other.0 == scene_both_key.0 => match &other.1 {
                SceneDialogs::SingleInline { dialogs, .. } => {
                    dialogs.contains_key(&scene_both_key.1)
                }
                SceneDialogs::DualInline { dialogs, .. } => dialogs.contains_key(&scene_both_key.1),
            },
            _ => false,
        }
    }
}

impl PartialEq<(&Cruid, &SceneDialogs)> for SceneId {
    fn eq(&self, other: &(&Cruid, &SceneDialogs)) -> bool {
        match self {
            SceneId::OnDemand(scene_usage) => scene_usage == other,
            SceneId::InMemory(scene_key) => scene_key == other,
        }
    }
}

/// Specify [on-demand](SceneId::OnDemand) scene usage.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SceneUsage {
    /// Used with [kira static sounds](https://docs.rs/kira/latest/kira/sound/static_sound/index.html).
    Static(SceneKey, PathBuf),
    /// Used with [kira streaming](https://docs.rs/kira/latest/kira/sound/streaming/index.html).
    Streaming(SceneKey, PathBuf),
}

impl AsRef<SceneKey> for SceneUsage {
    fn as_ref(&self) -> &SceneKey {
        match self {
            SceneUsage::Static(scene_key, ..) | SceneUsage::Streaming(scene_key, ..) => scene_key,
        }
    }
}

impl AsRef<Cruid> for SceneUsage {
    fn as_ref(&self) -> &Cruid {
        match self {
            SceneUsage::Static(scene_key, ..) | SceneUsage::Streaming(scene_key, ..) => {
                scene_key.as_ref()
            }
        }
    }
}

impl std::fmt::Display for SceneUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SceneUsage::Static(key, path) => write!(
                f,
                "static:{} ({})",
                key,
                path.display().to_string().as_str()
            ),
            SceneUsage::Streaming(key, path) => write!(
                f,
                "streaming:{} ({})",
                key,
                path.display().to_string().as_str()
            ),
        }
    }
}
