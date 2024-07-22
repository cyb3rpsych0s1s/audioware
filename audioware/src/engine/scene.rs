use std::{
    collections::HashMap,
    ops::DerefMut,
    sync::{Arc, Mutex, MutexGuard, OnceLock},
};

use kira::{
    manager::AudioManager,
    spatial::{
        emitter::{EmitterHandle, EmitterSettings},
        listener::{ListenerHandle, ListenerSettings},
        scene::{SpatialSceneHandle, SpatialSceneSettings},
    },
    OutputDestination,
};
use red4ext_rs::{
    log,
    types::{CName, EntityId, GameInstance},
    PluginOps,
};

use crate::{
    error::{Error, InternalError},
    types::{AsEntity, AsGameInstance},
    Audioware,
};

use super::{id::EmitterId, tracks::Tracks};

static SCENE: OnceLock<Scene> = OnceLock::new();

pub struct Scene {
    pub scene: Arc<Mutex<SpatialSceneHandle>>,
    pub v: Arc<Mutex<Option<ListenerHandle>>>,
    pub entities: Arc<Mutex<HashMap<EmitterId, EmitterHandle>>>,
}

impl Scene {
    pub fn setup(manager: &mut AudioManager) -> Result<(), Error> {
        let scene = manager
            .add_spatial_scene(SpatialSceneSettings::default())
            .map_err(|source| Error::Engine { source })?;
        SCENE
            .set(Scene {
                scene: Arc::new(Mutex::new(scene)),
                v: Arc::new(Mutex::new(None)),
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
    fn try_lock_listener<'a>() -> Result<MutexGuard<'a, Option<ListenerHandle>>, InternalError> {
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
        let v = Self::try_lock_scene()?
            .add_listener(
                position,
                orientation,
                ListenerSettings::new().track(&Tracks::get().v.main),
            )
            .map_err(|source| Error::Engine { source })?;
        *Self::try_lock_listener()?.deref_mut() = Some(v);
        log::info!(
            Audioware::env(),
            "registered listener: {:?} -> {:?}, {:?}",
            entity_id,
            position,
            orientation
        );
        Ok(())
    }
    pub fn unregister_listener(_: EntityId) -> Result<(), Error> {
        *Self::try_lock_listener()?.deref_mut() = None;
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
    pub fn clear_emitters() -> Result<(), Error> {
        Self::try_lock_emitters()?.clear();
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
}
