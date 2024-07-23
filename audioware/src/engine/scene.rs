use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard, OnceLock},
};

use glam::{Quat, Vec3};
use kira::{
    manager::AudioManager,
    spatial::{
        emitter::{EmitterHandle, EmitterSettings},
        listener::ListenerHandle,
        scene::{SpatialSceneHandle, SpatialSceneSettings},
    },
    tween::Tween,
    OutputDestination,
};
use red4ext_rs::{
    log,
    types::{CName, EntityId, GameInstance, Ref},
    PluginOps,
};

use crate::{
    error::{Error, InternalError},
    types::{AsEntity, AsGameInstance, Entity, Quaternion, Vector4},
    Audioware,
};

use super::id::EmitterId;

static SCENE: OnceLock<Scene> = OnceLock::new();

pub struct Scene {
    pub scene: Arc<Mutex<SpatialSceneHandle>>,
    pub v: Arc<Mutex<ListenerHandle>>,
    pub entities: Arc<Mutex<HashMap<EmitterId, EmitterHandle>>>,
}

impl Scene {
    pub fn setup(manager: &mut AudioManager) -> Result<(), Error> {
        let mut scene = manager
            .add_spatial_scene(SpatialSceneSettings::default())
            .map_err(|source| Error::Engine { source })?;
        let listener = scene.add_listener(Vec3::ZERO, Quat::IDENTITY, Default::default())?;
        SCENE
            .set(Scene {
                scene: Arc::new(Mutex::new(scene)),
                v: Arc::new(Mutex::new(listener)),
                entities: Arc::new(Mutex::new(HashMap::new())),
            })
            .map_err(|_| Error::from(InternalError::Contention { origin: "scene" }))?;
        Ok(())
    }
    fn try_lock_scene<'a>() -> Result<MutexGuard<'a, SpatialSceneHandle>, InternalError> {
        SCENE
            .get()
            .ok_or(InternalError::Init {
                origin: "spatial scene",
            })?
            .scene
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "spatial scene handle",
            })
    }
    fn try_lock_listener<'a>() -> Result<MutexGuard<'a, ListenerHandle>, InternalError> {
        SCENE
            .get()
            .ok_or(InternalError::Init {
                origin: "spatial scene",
            })?
            .v
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "spatial scene listener handle",
            })
    }
    fn try_lock_emitters<'a>(
    ) -> Result<MutexGuard<'a, HashMap<EmitterId, EmitterHandle>>, InternalError> {
        SCENE
            .get()
            .ok_or(InternalError::Init {
                origin: "spatial scene",
            })?
            .entities
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "spatial scene emitters handles",
            })
    }
    pub fn register_listener(entity_id: EntityId) -> Result<(), Error> {
        let game = GameInstance::new();
        let entity = GameInstance::find_entity_by_id(game, entity_id);
        let position = entity.get_world_position();
        let orientation = entity.get_world_orientation();
        // let transform = entity.get_world_transform();
        // log::info!(
        //     Audioware::env(),
        //     "transforms: {transform} -> {} (inverse)",
        //     transform.get_inverse()
        // );
        let mut v = Self::try_lock_listener()?;
        v.set_position(position, Tween::default());
        v.set_orientation(orientation, Tween::default());
        log::info!(
            Audioware::env(),
            "registered listener: {:?} -> {:?}, {:?}",
            entity_id,
            position,
            orientation
        );
        Ok(())
    }
    pub fn update_listener(position: Vector4, orientation: Quaternion) -> Result<(), Error> {
        let mut listener = Self::try_lock_listener()?;
        listener.set_position(position, Tween::default());
        listener.set_orientation(orientation, Tween::default());
        Ok(())
    }
    pub fn unregister_listener(_: EntityId) -> Result<(), Error> {
        Ok(())
    }
    pub fn register_emitter(entity_id: EntityId, emitter_name: Option<CName>) -> Result<(), Error> {
        let game = GameInstance::new();
        let entity = GameInstance::find_entity_by_id(game, entity_id);
        let position = entity.get_world_position();
        let emitter = Self::try_lock_scene()?
            .add_emitter(position, EmitterSettings::default())
            .map_err(|source| Error::Engine { source })?;
        Self::try_lock_emitters()?.insert(EmitterId::new(entity_id, emitter_name), emitter);
        log::info!(
            Audioware::env(),
            "registered emitter: {:?} -> {:?}",
            entity_id,
            position
        );
        Ok(())
    }
    pub fn update_emitter(id: &EntityId, position: Vector4) -> Result<(), Error> {
        let mut emitters = Self::try_lock_emitters()?;
        for (k, v) in emitters.iter_mut() {
            if k.entity_id() == id {
                v.set_position(position, Tween::default());
                break;
            }
        }
        Ok(())
    }
    pub fn unregister_emitter(entity_id: &EntityId) -> Result<(), Error> {
        let entities = Self::try_lock_emitters()?;
        let mut id: Option<&EmitterId> = None;
        for (k, _) in entities.iter() {
            if k.entity_id() == entity_id {
                id = Some(k);
                break;
            }
        }
        if let Some(id) = id {
            let mut entities = Self::try_lock_emitters()?;
            entities.remove(id);
        }
        Ok(())
    }
    pub fn emitters_count() -> Result<usize, Error> {
        Ok(Self::try_lock_emitters()?.len())
    }
    pub fn clear_emitters() -> Result<(), Error> {
        Self::try_lock_emitters()?.clear();
        Ok(())
    }
    pub fn sync_emitters() -> Result<(), Error> {
        let mut entity: Ref<Entity>;
        let mut position: Vector4;
        if let Ok(mut emitters) = Self::try_lock_emitters() {
            for (k, v) in emitters.iter_mut() {
                entity = GameInstance::find_entity_by_id(GameInstance::new(), *k.entity_id());
                if entity.is_null() {
                    continue;
                }
                position = entity.get_world_position();
                v.set_position(position, Tween::default());
            }
        }
        Ok(())
    }
    pub fn sync_listener() -> Result<(), Error> {
        if let Ok(ref mut v) = Self::try_lock_listener().as_deref_mut() {
            let player = GameInstance::get_player(GameInstance::new());
            if player.is_null() {
                return Ok(());
            }
            let entity = player.cast::<Entity>().unwrap();
            let position = entity.get_world_position();
            let orientation = entity.get_world_orientation();
            v.set_position(position, Tween::default());
            v.set_orientation(orientation, Tween::default());
        }
        Ok(())
    }
    pub fn output_destination(entity_id: &EntityId) -> Option<OutputDestination> {
        Self::try_lock_emitters().ok().and_then(|x| {
            x.iter().find_map(|(k, v)| {
                if k.entity_id() == entity_id {
                    Some(v.into())
                } else {
                    None
                }
            })
        })
    }
    pub fn is_registered_emitter(entity_id: &EntityId) -> bool {
        if let Ok(emitters) = Self::try_lock_emitters() {
            for k in emitters.keys() {
                if k.entity_id() == entity_id {
                    return true;
                }
            }
        }
        false
    }
}
