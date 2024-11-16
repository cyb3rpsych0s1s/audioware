use std::rc::Rc;

use bitflags::bitflags;
use dashmap::DashMap;
use kira::{
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle},
    spatial::{emitter::EmitterHandle, listener::ListenerHandle, scene::SpatialSceneId},
    track::{TrackHandle, TrackId},
};
use red4ext_rs::types::{CName, EntityId};
use snowflake::ProcessUniqueId;

pub struct Handles<T>(DashMap<ProcessUniqueId, T>);

pub struct DualHandles<E> {
    statics: Handles<StaticSoundHandle>,
    streams: Handles<StreamingSoundHandle<E>>,
}

pub struct CustomTrack<E> {
    pub handle: Rc<TrackHandle>,
    pub handles: DualHandles<E>,
}

impl<E> CustomTrack<E> {
    fn track_handle(&self) -> &TrackHandle {
        &self.handle
    }
}

#[rustfmt::skip]
bitflags! {
    pub struct EmitterFlags: u8 {
        const DEAD      = 1 << 0;
        const DEFEATED  = 1 << 1;
        const BUSY      = 1 << 2;
    }
}

pub struct CustomEmitter {
    pub id: EntityId,
    pub name: Option<CName>,
    pub handle: EmitterHandle,
    pub flags: EmitterFlags,
    // pub kind: NPCType,
}

pub struct CustomListener {
    pub id: EntityId,
    pub handle: ListenerHandle,
    pub route: TrackId,
}

pub struct CustomScene {
    pub listeners: Handles<CustomListener>,
    pub emitters: Handles<CustomEmitter>,
}

impl CustomScene {
    fn track_ids(&self) -> Vec<TrackId> {
        self.listeners.0.iter().map(|x| x.value().route).collect()
    }
}

pub struct CustomEngine<E> {
    pub tracks: DashMap<TrackId, CustomTrack<E>>,
    pub scenes: DashMap<SpatialSceneId, CustomScene>,
}

impl<E> CustomEngine<E> {
    fn scene_tracks(&self, id: &SpatialSceneId) -> Vec<Rc<TrackHandle>> {
        if let Some(scene) = self.scenes.iter().find(|x| x.key() == id) {
            let ids = scene.value().track_ids();
            return self
                .tracks
                .iter()
                .filter(|x| ids.contains(x.key()))
                .map(|x| x.value().handle.clone())
                .collect();
        }
        vec![]
    }
}
