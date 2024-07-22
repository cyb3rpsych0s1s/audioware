use std::{
    collections::HashMap,
    ops::DerefMut,
    sync::{Arc, Mutex, OnceLock},
};

use kira::{
    manager::AudioManager,
    spatial::{
        emitter::EmitterHandle,
        listener::{ListenerHandle, ListenerSettings},
        scene::{SpatialSceneHandle, SpatialSceneSettings},
    },
};
use red4ext_rs::types::{EntityId, GameInstance};

use crate::{
    error::{Error, InternalError},
    types::{AsEntity, AsGameInstance},
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
    pub fn register_listener(entity_id: EntityId) -> Result<(), Error> {
        let game = GameInstance::new();
        let entity = GameInstance::find_entity_by_id(game, entity_id);
        let position = entity.get_world_position();
        let orientation = entity.get_world_orientation();
        let scene = SCENE.get().unwrap();
        let v = scene
            .scene
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "spatial scene",
            })?
            .add_listener(
                position,
                orientation,
                ListenerSettings::new().track(&Tracks::get().v.main),
            )
            .map_err(|source| Error::Engine { source })?;
        *scene
            .v
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "write spatial scene listener",
            })?
            .deref_mut() = Some(v);
        Ok(())
    }
    pub fn unregister_listener(_: EntityId) -> Result<(), Error> {
        let scene = SCENE.get().unwrap();
        *scene
            .v
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "erase spatial scene listener",
            })?
            .deref_mut() = None;
        Ok(())
    }
}
