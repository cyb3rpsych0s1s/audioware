use dashmap::DashMap;
use kira::{
    manager::{backend::Backend, AudioManager},
    spatial::{
        emitter::{EmitterHandle, EmitterSettings},
        listener::{ListenerHandle, ListenerSettings},
        scene::{SpatialSceneHandle, SpatialSceneSettings},
    },
};
use red4ext_rs::types::{CName, EntityId, GameInstance};

use crate::{
    error::{Error, SceneError},
    get_player, AIActionHelper, AsEntity, AsGameInstance, Entity, GameObject, Vector4,
};

use super::{lifecycle, tracks::Tracks, tweens::IMMEDIATELY};

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
        tracks: &Tracks,
    ) -> Result<Self, Error> {
        let settings = SpatialSceneSettings::default();
        let capacity = settings.emitter_capacity as usize;
        let mut scene = manager.add_spatial_scene(settings)?;
        let (listener_id, position, orientation) = {
            let v = get_player(GameInstance::new()).cast::<Entity>().unwrap();
            (
                v.get_entity_id(),
                v.get_world_position(),
                v.get_world_orientation(),
            )
        };
        let v = scene.add_listener(
            position,
            orientation,
            ListenerSettings::default().track(tracks.sfx.as_ref()),
        )?;
        Ok(Self {
            v,
            listener_id,
            scene,
            emitters: DashMap::with_capacity(capacity),
        })
    }

    fn emitter_infos(&self, entity_id: EntityId) -> Result<(Vector4, bool), Error> {
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
        emitter_name: Option<CName>,
        settings: Option<EmitterSettings>,
    ) -> Result<(), Error> {
        if entity_id == self.listener_id {
            return Err(Error::Scene {
                source: SceneError::InvalidEmitter,
            });
        }
        let (position, busy) = self.emitter_infos(entity_id)?;
        let emitter = self
            .scene
            .add_emitter(position, settings.unwrap_or_default())?;
        let handle = Handle {
            handle: emitter,
            last_known_position: position,
            busy,
            dead: false,
        };
        self.emitters
            .insert(EmitterId::new(entity_id, emitter_name), handle);
        lifecycle!("added emitter {entity_id:?}");
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
        lifecycle!("removed emitter {entity_id:?}");
        Ok(true)
    }

    fn sync_listener(&mut self) -> Result<(), Error> {
        let player = get_player(GameInstance::new());
        if player.is_null() {
            return Ok(());
        }
        let (position, orientation) = {
            let entity = player.cast::<Entity>().unwrap();
            (entity.get_world_position(), entity.get_world_orientation())
        };
        self.v.set_position(position, IMMEDIATELY);
        self.v.set_orientation(orientation, IMMEDIATELY);
        Ok(())
    }

    fn sync_emitters(&mut self) -> Result<(), Error> {
        if self.emitters.is_empty() {
            return Ok(());
        }
        self.emitters.retain(|k, v| {
            if v.dead {
                return false;
            }
            let Ok((position, busy)) = self.emitter_infos(k.id) else {
                return false;
            };
            v.busy = busy;
            v.last_known_position = position;
            // weirdly enough if emitter is not updated, sound(s) won't update as expected.
            // e.g. when listener moves but emitter stands still.
            v.handle.set_position(position, IMMEDIATELY);
            true
        });
        Ok(())
    }

    pub fn sync(&mut self) -> Result<(), Error> {
        self.sync_listener()?;
        self.sync_emitters()?;
        Ok(())
    }

    pub fn is_registered_emitter(&self, entity_id: EntityId) -> bool {
        for pair in self.emitters.iter() {
            if pair.key().id == entity_id {
                return true;
            }
        }
        false
    }

    pub fn on_emitter_dies(&mut self, entity_id: EntityId) {
        for ref mut emitter in self.emitters.iter_mut() {
            if emitter.key().id == entity_id {
                emitter.value_mut().dead = true;
            }
        }
    }

    pub fn any_emitter(&self) -> bool {
        !self.emitters.is_empty()
    }

    pub fn clear(&mut self) {
        self.emitters.clear();
    }
}

/// Represents a currently registered spatial audio scene emitter.
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

#[derive(Debug)]
pub struct Handle {
    handle: EmitterHandle,
    last_known_position: Vector4,
    busy: bool,
    dead: bool,
}

impl Handle {
    pub fn handle(&self) -> &EmitterHandle {
        &self.handle
    }
}
