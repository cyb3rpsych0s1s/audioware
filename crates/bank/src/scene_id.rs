use crate::{SceneKey, Usage};

/// Special type whose audio data is guaranteed to both exist in banks and be valid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SceneId {
    OnDemand(Usage),
    InMemory(SceneKey),
}
