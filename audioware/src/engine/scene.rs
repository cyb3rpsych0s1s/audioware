use dashmap::DashMap;
use glam::{Quat, Vec3};
use kira::{
    manager::{backend::Backend, AudioManager},
    spatial::{
        emitter::EmitterHandle,
        listener::{ListenerHandle, ListenerSettings},
        scene::{SpatialSceneHandle, SpatialSceneSettings},
    },
    track::TrackHandle,
};
use red4ext_rs::types::{CName, EntityId, Opt};

use crate::{error::Error, EmitterSettings};

/// Audio spatial scene.
pub struct Scene {
    pub emitters: DashMap<EmitterId, EmitterHandle>,
    pub dead_emitters: Vec<EntityId>,
    pub v: ListenerHandle,
    pub scene: SpatialSceneHandle,
}

impl Scene {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        track: &TrackHandle,
    ) -> Result<Self, Error> {
        let settings = SpatialSceneSettings::default();
        let capacity = settings.emitter_capacity as usize;
        let mut scene = manager.add_spatial_scene(settings)?;
        let v = scene.add_listener(
            Vec3::ZERO,
            Quat::IDENTITY,
            ListenerSettings::default().track(track),
        )?;
        Ok(Self {
            v,
            scene,
            emitters: DashMap::with_capacity(capacity),
            dead_emitters: Vec::with_capacity(capacity),
        })
    }

    pub fn add_emitter(
        &mut self,
        position: Vec3,
        entity_id: EntityId,
        emitter_name: Opt<CName>,
        settings: Opt<EmitterSettings>,
    ) -> Result<(), Error> {
        let emitter = self.scene.add_emitter(
            position,
            settings.into_option().map(Into::into).unwrap_or_default(),
        )?;
        self.emitters.insert(
            EmitterId::new(entity_id, emitter_name.into_option()),
            emitter,
        );
        Ok(())
    }

    pub fn remove_emitter(&mut self, entity_id: EntityId) -> Result<bool, Error> {
        let removal: Vec<_> = self
            .emitters
            .iter()
            .filter_map(|x| {
                if x.key().id == entity_id {
                    Some(x.key().clone())
                } else {
                    None
                }
            })
            .collect();
        if removal.is_empty() {
            return Ok(false);
        }
        self.emitters.retain(|k, _| !removal.contains(k));
        Ok(true)
    }
}

/// Represents a currently registered spatial audio scene
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmitterId {
    id: EntityId,
    name: Option<CName>,
}

impl EmitterId {
    pub fn new(id: EntityId, name: Option<CName>) -> Self {
        Self { id, name }
    }
}
