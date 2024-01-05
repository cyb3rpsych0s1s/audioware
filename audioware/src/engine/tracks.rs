use std::{
    collections::HashMap,
    mem::MaybeUninit,
    sync::{atomic::AtomicBool, Arc, Mutex, OnceLock},
};

use audioware_types::interop::game::get_game_instance;
use glam::{Quat, Vec3};
use kira::{
    manager::AudioManager,
    spatial::{
        emitter::{EmitterHandle, EmitterSettings},
        listener::{ListenerHandle, ListenerSettings},
        scene::{SpatialSceneHandle, SpatialSceneSettings},
    },
    track::{effect::reverb::ReverbBuilder, TrackBuilder, TrackHandle, TrackRoutes},
    OutputDestination,
};
use lazy_static::lazy_static;
use red4ext_rs::types::{CName, EntityId};

use super::id::SoundEntityId;

lazy_static! {
    static ref TRACKS: OnceLock<Tracks> = OnceLock::default();
    static ref SCENE: Scene = Scene::default();
}

#[allow(dead_code)]
struct Tracks {
    reverb: TrackHandle,
    v: V,
}

#[allow(dead_code)]
struct V {
    main: TrackHandle,
    vocal: TrackHandle,
    mental: TrackHandle,
    emissive: TrackHandle,
}

struct Scene {
    scene: Arc<Mutex<MaybeUninit<SpatialSceneHandle>>>,
    v: Arc<Mutex<MaybeUninit<ListenerHandle>>>,
    entities: Arc<Mutex<HashMap<SoundEntityId, EmitterHandle>>>,
    initialized: AtomicBool,
}

impl Drop for Scene {
    fn drop(&mut self) {
        if self.initialized.load(std::sync::atomic::Ordering::SeqCst) {
            if let Ok(mut guard) = self.scene.clone().try_lock() {
                unsafe {
                    guard.assume_init_drop();
                }
            }
            if let Ok(mut guard) = self.v.clone().try_lock() {
                unsafe {
                    guard.assume_init_drop();
                }
            }
        }
        if let Ok(entities) = self.entities.clone().try_lock() {
            std::mem::drop(entities);
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            scene: Arc::new(Mutex::new(MaybeUninit::uninit())),
            v: Arc::new(Mutex::new(MaybeUninit::uninit())),
            entities: Default::default(),
            initialized: Default::default(),
        }
    }
}

pub fn setup(manager: &mut AudioManager) -> anyhow::Result<()> {
    let reverb = manager.add_sub_track({
        let mut builder = TrackBuilder::new();
        builder.add_effect(ReverbBuilder::new().mix(1.0));
        builder
    })?;
    let mut scene = manager.add_spatial_scene(SpatialSceneSettings::default())?;
    let v = scene.add_listener(Vec3::ZERO, Quat::IDENTITY, ListenerSettings::default())?;
    let main = manager
        .add_sub_track(TrackBuilder::new().routes(TrackRoutes::new().with_route(&reverb, 0.25)))?;
    let vocal = manager
        .add_sub_track(TrackBuilder::new().routes(TrackRoutes::new().with_route(&main, 1.)))?;
    let mental = manager
        .add_sub_track(TrackBuilder::new().routes(TrackRoutes::new().with_route(&main, 1.)))?;
    let emissive = manager
        .add_sub_track(TrackBuilder::new().routes(TrackRoutes::new().with_route(&main, 1.)))?;
    TRACKS
        .set(Tracks {
            reverb,
            v: V {
                main,
                vocal,
                mental,
                emissive,
            },
        })
        .map_err(|_| anyhow::anyhow!("error setting audio engine tracks"))?;
    {
        SCENE
            .scene
            .clone()
            .try_lock()
            .map_err(|_| anyhow::anyhow!("error setting audio engine scene"))?
            .write(scene);
        SCENE
            .v
            .clone()
            .try_lock()
            .map_err(|_| anyhow::anyhow!("error setting audio engine scene"))?
            .write(v);
        SCENE
            .initialized
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn output_destination<'a>(
    entity_id: Option<EntityId>,
    emitter_name: Option<CName>,
) -> Option<OutputDestination> {
    let is_player = entity_id
        .clone()
        .and_then(|x| {
            let gi = get_game_instance();
            let entity = audioware_types::interop::game::find_entity_by_id(gi, x);
            entity.into_ref().map(|entity| entity.is_player())
        })
        .unwrap_or(false);
    match (entity_id, emitter_name, is_player) {
        (Some(_), Some(_), true) => TRACKS
            .get()
            .map(|x| &x.v.vocal)
            .map(OutputDestination::from),
        (Some(_), None, true) => TRACKS
            .get()
            .map(|x| &x.v.emissive)
            .map(OutputDestination::from),
        (Some(id), _, false) => TRACKS.get().and_then(|_x| {
            SCENE
                .entities
                .clone()
                .try_lock()
                .ok()
                .and_then(|x| x.get(&SoundEntityId(id)).map(OutputDestination::from))
        }),
        (None, _, _) => TRACKS.get().map(|x| &x.v.main).map(OutputDestination::from),
    }
}

pub fn register_emitter(id: EntityId) {
    let key = SoundEntityId(id.clone());
    if let (Ok(mut scene), Ok(mut entities)) = (
        SCENE.scene.clone().try_lock(),
        SCENE.entities.clone().try_lock(),
    ) {
        if let std::collections::hash_map::Entry::Vacant(e) = entities.entry(key) {
            let entity = audioware_types::interop::game::find_entity_by_id(get_game_instance(), id);
            if let Some(entity) = entity.into_ref() {
                let position = entity.get_world_position();
                let position: Vec3 = position.into();
                if let Ok(handle) = unsafe { scene.assume_init_mut() }
                    .add_emitter(position, EmitterSettings::default())
                {
                    e.insert(handle);
                }
            }
        }
    }
}

pub fn unregister_emitter(id: EntityId) {
    let key = SoundEntityId(id.clone());
    if let Ok(mut entities) = SCENE.entities.clone().try_lock() {
        if entities.contains_key(&key) {
            entities.remove(&key);
        }
    }
}
