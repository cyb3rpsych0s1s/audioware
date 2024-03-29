use std::{
    collections::HashMap,
    mem::MaybeUninit,
    sync::{atomic::AtomicBool, Arc, Mutex, OnceLock},
};

use audioware_sys::interop::{game::get_game_instance, quaternion::Quaternion, vector4::Vector4};
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

use crate::types::id::SoundEntityId;

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
    let main = manager
        .add_sub_track(TrackBuilder::new().routes(TrackRoutes::new().with_route(&reverb, 0.)))?;
    let v = scene.add_listener(
        Vec3::ZERO,
        Quat::IDENTITY,
        ListenerSettings::new().track(&main),
    )?;
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
            .map_err(|_| anyhow::anyhow!("error setting audio engine listener"))?
            .write(v);
        SCENE
            .initialized
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn output_destination(
    entity_id: Option<EntityId>,
    emitter_name: Option<CName>,
) -> Option<OutputDestination> {
    let is_player = entity_id
        .clone()
        .and_then(|x| {
            let gi = get_game_instance();
            let entity = audioware_sys::interop::game::find_entity_by_id(gi, x);
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
        (Some(id), _, false) => {
            red4ext_rs::info!(
                "retrieving entity id from scene ({})",
                u64::from(id.clone())
            );
            SCENE
                .entities
                .clone()
                .try_lock()
                .ok()
                .and_then(|x| x.get(&SoundEntityId(id)).map(OutputDestination::from))
        }
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
            let entity =
                audioware_sys::interop::game::find_entity_by_id(get_game_instance(), id.clone());
            if let Some(entity) = entity.into_ref() {
                let position = entity.get_world_position();
                if let Ok(handle) = unsafe { scene.assume_init_mut() }
                    .add_emitter(position, EmitterSettings::default())
                {
                    e.insert(handle);
                    red4ext_rs::info!("register emitter ({})", u64::from(id.clone()));
                } else {
                    red4ext_rs::info!("unable to add emitter to scene ({})", u64::from(id.clone()));
                }
            } else {
                red4ext_rs::error!("unable to find entity ({id:#?})");
            }
        } else {
            red4ext_rs::error!("entry not vacant ({id:#?})");
        }
    } else {
        red4ext_rs::error!("unable to reach scene and its entities");
    }
}

pub fn unregister_emitter(id: EntityId) {
    let key = SoundEntityId(id.clone());
    if let Ok(mut entities) = SCENE.entities.clone().try_lock() {
        if entities.contains_key(&key) {
            entities.remove(&key);
            red4ext_rs::info!("unregister emitter ({})", u64::from(id.clone()));
        }
    } else {
        red4ext_rs::error!("unable to get scene entities");
    }
}

pub fn update_listener(position: Vector4, orientation: Quaternion) {
    if let Ok(mut listener) = SCENE.v.clone().try_lock() {
        if let Err(e) =
            unsafe { listener.assume_init_mut() }.set_position(position.clone(), Default::default())
        {
            red4ext_rs::error!("error setting listener position: {e:#?}");
        }
        if let Err(e) = unsafe { listener.assume_init_mut() }
            .set_orientation(orientation.clone(), Default::default())
        {
            red4ext_rs::error!("error setting listener orientation: {e:#?}");
        }
        // red4ext_rs::info!(
        //     "update listener position to {}, {}, {} / orientation to {}, {}, {}, {}",
        //     position.x,
        //     position.y,
        //     position.z,
        //     orientation.i,
        //     orientation.j,
        //     orientation.k,
        //     orientation.r
        // );
    } else {
        red4ext_rs::error!("unable to get scene listener");
    }
}

pub fn update_emitter(id: EntityId, position: Vector4) {
    let key = SoundEntityId(id.clone());
    if let Ok(mut guard) = SCENE.entities.clone().try_lock() {
        if let Some(emitter) = guard.get_mut(&key) {
            if let Err(e) = emitter.set_position(position.clone(), Default::default()) {
                red4ext_rs::error!(
                    "unable to set emitter position: {e} ({})",
                    u64::from(id.clone())
                );
            } else {
                // red4ext_rs::info!(
                //     "update emitter ({}) position to {}, {}, {}",
                //     u64::from(id.clone()),
                //     position.x,
                //     position.y,
                //     position.z
                // );
            }
        } else {
            red4ext_rs::error!("unable to get scene emitter ({})", u64::from(id.clone()));
        }
    } else {
        red4ext_rs::error!("unable to get scene entities");
    }
}

pub fn emitters_count() -> i32 {
    if let Ok(guard) = SCENE.entities.clone().try_lock() {
        return guard.len() as i32;
    }
    -1
}

pub fn update_player_reverb(value: f32) -> bool {
    if value > 1. {
        red4ext_rs::error!("reverb must be between 0. and 1. (inclusive)");
        return false;
    }
    if let Some(tracks) = TRACKS.get() {
        if let Ok(()) = tracks.v.main.set_route(
            &tracks.reverb,
            kira::Volume::Amplitude(value as f64),
            Default::default(),
        ) {
            return true;
        }
        return false;
    }
    false
}
