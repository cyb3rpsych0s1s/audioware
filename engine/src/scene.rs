use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use audioware_core::UninitializedSnafu;
use glam::{Quat, Vec3};
use kira::{
    manager::AudioManager,
    spatial::{
        emitter::EmitterHandle,
        listener::{ListenerHandle, ListenerSettings},
        scene::{SpatialSceneHandle, SpatialSceneSettings},
    },
};
use once_cell::sync::OnceCell;
use snafu::OptionExt;

use super::{error::Error, id::SoundEntityId, track::maybe_tracks};

pub static SCENE: OnceCell<Scene> = OnceCell::new();

#[inline(always)]
pub fn maybe_scene<'cell>() -> Result<&'cell Scene, Error> {
    Ok(SCENE.get().context(UninitializedSnafu { which: "scene" })?)
}

#[inline(always)]
pub fn maybe_scene_entities<'mutex>(
) -> Result<MutexGuard<'mutex, HashMap<SoundEntityId, EmitterHandle>>, Error> {
    maybe_scene()?
        .entities
        .try_lock()
        .map_err(|e| Error::Internal { source: e.into() })
}

#[allow(dead_code)]
pub struct Scene {
    pub scene: Arc<Mutex<SpatialSceneHandle>>,
    pub v: Arc<Mutex<ListenerHandle>>,
    pub entities: Arc<Mutex<HashMap<SoundEntityId, EmitterHandle>>>,
}

impl Scene {
    pub fn setup(manager: &mut AudioManager) -> Result<(), Error> {
        let mut scene = manager.add_spatial_scene(SpatialSceneSettings::default())?;
        let v = scene.add_listener(
            Vec3::ZERO,
            Quat::IDENTITY,
            ListenerSettings::new().track(&maybe_tracks()?.v.main),
        )?;
        SCENE
            .set(Scene {
                scene: Arc::new(Mutex::new(scene)),
                v: Arc::new(Mutex::new(v)),
                entities: Arc::new(Mutex::new(HashMap::new())),
            })
            .map_err(|_| Error::Internal {
                source: audioware_core::Error::CannotSet { which: "scene" },
            })?;
        Ok(())
    }
}
