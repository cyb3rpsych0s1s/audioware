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
use red4ext_rs::types::{CName, EntityId, GameInstance, Opt};

use crate::{
    error::{Error, SceneError},
    get_player, AIActionHelper, AsEntity, AsGameInstance, EmitterSettings, Entity, GameObject,
    Vector4,
};

/// Audio spatial scene.
pub struct Scene {
    pub emitters: DashMap<EmitterId, Handle>,
    pub v: ListenerHandle,
    pub listener_id: EntityId,
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
        let v = get_player(GameInstance::new());
        let listener_id = v.cast::<Entity>().map(|x| x.get_entity_id()).unwrap();
        let v = scene.add_listener(
            Vec3::ZERO,
            Quat::IDENTITY,
            ListenerSettings::default().track(track),
        )?;
        Ok(Self {
            v,
            listener_id,
            scene,
            emitters: DashMap::with_capacity(capacity),
        })
    }

    fn emitter_infos(&self, entity_id: EntityId) -> Result<(Vector4, bool), Error> {
        if !entity_id.is_defined() {
            return Err(Error::Scene {
                source: SceneError::InvalidEmitter,
            });
        }
        let game = GameInstance::new();
        let entity = GameInstance::find_entity_by_id(game, entity_id);
        if entity.is_null() {
            return Err(Error::Scene {
                source: SceneError::InvalidEmitter,
            });
        }
        let busy = if entity.is_a::<GameObject>() {
            AIActionHelper::is_in_workspot(entity.clone().cast::<GameObject>().unwrap())
        } else {
            false
        };
        let position = entity.get_world_position();
        Ok((position, busy))
    }

    pub fn add_emitter(
        &mut self,
        entity_id: EntityId,
        emitter_name: Opt<CName>,
        settings: Opt<EmitterSettings>,
    ) -> Result<(), Error> {
        if entity_id == self.listener_id {
            return Err(Error::Scene {
                source: SceneError::InvalidEmitter,
            });
        }
        let (position, busy) = self.emitter_infos(entity_id)?;
        let emitter = self.scene.add_emitter(
            position,
            settings.into_option().map(Into::into).unwrap_or_default(),
        )?;
        let handle = Handle {
            handle: emitter,
            last_known_position: position,
            busy,
        };
        self.emitters.insert(
            EmitterId::new(entity_id, emitter_name.into_option()),
            handle,
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

    fn sync_listener(&mut self) -> Result<(), Error> {
        let player = get_player(GameInstance::new());
        if player.is_null() {
            return Ok(());
        }
        let entity = player.cast::<Entity>().unwrap();
        let position = entity.get_world_position();
        let orientation = entity.get_world_orientation();
        self.v.set_position(position, Default::default());
        self.v.set_orientation(orientation, Default::default());
        Ok(())
    }

    fn sync_emitters(&mut self) -> Result<(), Error> {
        if self.emitters.is_empty() {
            return Ok(());
        }
        self.emitters.retain(|k, v| {
            let game = GameInstance::new();
            let entity = GameInstance::find_entity_by_id(game, k.id);
            if entity.is_null() {
                return false;
            }
            v.busy = entity.is_in_workspot();
            v.last_known_position = entity.get_world_position();
            v.handle
                .set_position(v.last_known_position, Default::default());
            true
        });
        Ok(())
    }

    pub fn sync(&mut self) -> Result<(), Error> {
        self.sync_listener()?;
        self.sync_emitters()?;
        Ok(())
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

pub struct Handle {
    handle: EmitterHandle,
    last_known_position: Vector4,
    busy: bool,
}

impl Handle {
    pub fn handle(&self) -> &EmitterHandle {
        &self.handle
    }
}
