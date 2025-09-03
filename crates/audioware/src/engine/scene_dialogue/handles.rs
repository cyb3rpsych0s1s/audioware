use ahash::HashMap;
use kira::sound::{
    FromFileError, PlaybackState, static_sound::StaticSoundHandle, streaming::StreamingSoundHandle,
};
use red4ext_rs::types::{Cruid, EntityId};

#[derive(Debug, Default)]
pub struct SceneHandles {
    pub statics: HashMap<Cruid, SceneHandle<StaticSoundHandle>>,
    pub streams: HashMap<Cruid, SceneHandle<StreamingSoundHandle<FromFileError>>>,
}

#[derive(Debug)]
pub struct SceneHandle<T> {
    entity_id: Option<EntityId>,
    handle: T,
}

impl SceneHandles {
    pub fn is_v_speaking(&self) -> bool {
        self.statics
            .iter()
            .any(|(_, x)| x.entity_id.is_none() && x.handle.state() == PlaybackState::Playing)
            || self
                .streams
                .iter()
                .any(|(_, x)| x.entity_id.is_none() && x.handle.state() == PlaybackState::Playing)
    }
    pub fn is_entity_speaking(&self, entity_id: &EntityId) -> bool {
        self.statics.iter().any(|(_, x)| {
            x.entity_id.map(|x| x == *entity_id).unwrap_or(false)
                && x.handle.state() == PlaybackState::Playing
        }) || self.streams.iter().any(|(_, x)| {
            x.entity_id.map(|x| x == *entity_id).unwrap_or(false)
                && x.handle.state() == PlaybackState::Playing
        })
    }
}
