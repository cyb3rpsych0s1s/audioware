use std::{
    collections::HashMap,
    ops::DerefMut,
    sync::{Arc, Mutex, OnceLock},
};

use glam::{Quat, Vec3};
use kira::{
    manager::AudioManager,
    spatial::{
        emitter::EmitterHandle,
        listener::{ListenerHandle, ListenerSettings},
        scene::{SpatialSceneHandle, SpatialSceneSettings},
    },
};
use red4ext_rs::types::EntityId;

use crate::error::{Error, InternalError};

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
        let scene = SCENE.get().unwrap();
        let v = scene
            .scene
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: "spatial scene",
            })?
            .add_listener(
                Vec3::ZERO,
                Quat::IDENTITY,
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
}
