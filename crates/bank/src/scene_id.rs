use std::path::PathBuf;

use audioware_manifest::SceneDialog;
use red4ext_rs::types::Cruid;

use crate::SceneKey;

/// Special type whose audio data is guaranteed to both exist in banks and be valid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SceneId {
    OnDemand(SceneUsage),
    InMemory(SceneKey),
}

impl AsRef<SceneKey> for SceneId {
    fn as_ref(&self) -> &SceneKey {
        match self {
            Self::OnDemand(scene_usage) => scene_usage.as_ref(),
            Self::InMemory(scene_key) => scene_key,
        }
    }
}

impl PartialEq<SceneUsage> for SceneId {
    fn eq(&self, other: &SceneUsage) -> bool {
        match self {
            SceneId::OnDemand(scene_usage) => scene_usage == other,
            SceneId::InMemory(scene_key) => scene_key == other.as_ref(),
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

impl PartialEq<(&Cruid, &SceneDialog)> for SceneUsage {
    fn eq(&self, other: &(&Cruid, &SceneDialog)) -> bool {
        match self {
            SceneUsage::Static(scene_key, ..) | SceneUsage::Streaming(scene_key, ..) => {
                scene_key == other
            }
        }
    }
}

impl PartialEq<(&Cruid, &SceneDialog)> for SceneKey {
    fn eq(&self, other: &(&Cruid, &SceneDialog)) -> bool {
        match self {
            SceneKey::Locale(scene_locale_key) if *other.0 == scene_locale_key.0 => {
                match &other.1 {
                    SceneDialog::SingleInline { dialogs, .. } => {
                        dialogs.contains_key(&scene_locale_key.1)
                    }
                    SceneDialog::DualInline { dialogs, .. } => {
                        dialogs.contains_key(&scene_locale_key.1)
                    }
                }
            }
            SceneKey::Both(scene_both_key) if *other.0 == scene_both_key.0 => match &other.1 {
                SceneDialog::SingleInline { dialogs, .. } => {
                    dialogs.contains_key(&scene_both_key.1)
                }
                SceneDialog::DualInline { dialogs, .. } => dialogs.contains_key(&scene_both_key.1),
            },
            _ => false,
        }
    }
}

impl PartialEq<(&Cruid, &SceneDialog)> for SceneId {
    fn eq(&self, other: &(&Cruid, &SceneDialog)) -> bool {
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
