use std::{
    collections::HashMap,
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

use crate::error::{Error, InternalError};

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
        let v = scene
            .add_listener(
                Vec3::ZERO,
                Quat::IDENTITY,
                // ListenerSettings::new().track(&maybe_tracks()?.v.main),
                ListenerSettings::default(),
            )
            .map_err(|source| Error::Engine { source })?;
        SCENE
            .set(Scene {
                scene: Arc::new(Mutex::new(scene)),
                v: Arc::new(Mutex::new(v)),
                entities: Arc::new(Mutex::new(HashMap::new())),
            })
            .map_err(|_| Error::from(InternalError::Contention { origin: "scene" }))?;
        Ok(())
    }
}
