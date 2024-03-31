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
    track::{
        effect::{
            filter::{FilterBuilder, FilterHandle, FilterMode},
            reverb::ReverbBuilder,
        },
        TrackBuilder, TrackHandle, TrackRoutes,
    },
    OutputDestination,
};
use lazy_static::lazy_static;
use red4ext_rs::types::{CName, EntityId};

use crate::types::id::SoundEntityId;

use super::effects::{
    EqPass, HighPass, LowPass, Preset, EQ, EQ_HIGH_PASS_PHONE_CUTOFF, EQ_LOW_PASS_PHONE_CUTOFF,
    EQ_RESONANCE,
};

lazy_static! {
    static ref TRACKS: OnceLock<Tracks> = OnceLock::default();
    static ref SCENE: Scene = Scene::default();
}

#[allow(dead_code)]
struct Tracks {
    reverb: TrackHandle,
    v: V,
    holocall: Holocall,
}

#[allow(dead_code)]
struct V {
    main: TrackHandle,
    vocal: TrackHandle,
    mental: TrackHandle,
    emissive: TrackHandle,
    eq: Mutex<EQ>,
}

#[allow(dead_code)]
struct Holocall {
    main: TrackHandle,
    eq: Mutex<EQ>,
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
    let player_lowpass: FilterHandle;
    let player_highpass: FilterHandle;
    let holocall_lowpass: FilterHandle;
    let holocall_highpass: FilterHandle;
    let mut scene = manager.add_spatial_scene(SpatialSceneSettings::default())?;
    let main = manager.add_sub_track(
        {
            let mut builder = TrackBuilder::new();
            player_lowpass = builder.add_effect(FilterBuilder::default().mix(0.));
            player_highpass =
                builder.add_effect(FilterBuilder::default().mode(FilterMode::HighPass).mix(0.));
            builder
        }
        .routes(TrackRoutes::new().with_route(&reverb, 0.)),
    )?;
    let holocall = manager.add_sub_track({
        let mut builder = TrackBuilder::new();
        holocall_lowpass = builder.add_effect(
            FilterBuilder::default()
                .cutoff(EQ_LOW_PASS_PHONE_CUTOFF)
                .resonance(EQ_RESONANCE),
        );
        holocall_highpass = builder.add_effect(
            FilterBuilder::default()
                .mode(FilterMode::HighPass)
                .cutoff(EQ_HIGH_PASS_PHONE_CUTOFF)
                .resonance(EQ_RESONANCE),
        );
        builder
    })?;
    let eq = EQ {
        lowpass: LowPass(player_lowpass),
        highpass: HighPass(player_highpass),
    };
    let v = scene.add_listener(
        Vec3::ZERO,
        Quat::IDENTITY,
        ListenerSettings::new().track(&main),
    )?;
    let vocal = manager.add_sub_track(TrackBuilder::new().routes(TrackRoutes::parent(&main)))?;
    let mental = manager.add_sub_track(TrackBuilder::new().routes(TrackRoutes::parent(&main)))?;
    let emissive = manager.add_sub_track(TrackBuilder::new().routes(TrackRoutes::parent(&main)))?;
    TRACKS
        .set(Tracks {
            reverb,
            v: V {
                main,
                vocal,
                mental,
                emissive,
                eq: Mutex::new(eq),
            },
            holocall: Holocall {
                main: holocall,
                eq: Mutex::new(EQ {
                    lowpass: LowPass(holocall_lowpass),
                    highpass: HighPass(holocall_highpass),
                }),
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
    over_the_phone: bool,
) -> Option<OutputDestination> {
    let is_player = entity_id
        .clone()
        .and_then(|x| {
            let gi = get_game_instance();
            let entity = audioware_sys::interop::game::find_entity_by_id(gi, x);
            entity.into_ref().map(|entity| entity.is_player())
        })
        .unwrap_or(false);
    match (entity_id, emitter_name, is_player, over_the_phone) {
        (Some(_), Some(_), true, _) => TRACKS
            .get()
            .map(|x| &x.v.vocal)
            .map(OutputDestination::from),
        (Some(_), None, true, _) => TRACKS
            .get()
            .map(|x| &x.v.emissive)
            .map(OutputDestination::from),
        (Some(id), _, false, _) => {
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
        (None, Some(_), false, true) => TRACKS
            .get()
            .map(|x| &x.holocall.main)
            .map(OutputDestination::from),
        (None, _, _, _) => TRACKS.get().map(|x| &x.v.main).map(OutputDestination::from),
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
        red4ext_rs::warn!("unable to update reverb route volume");
        return false;
    }
    red4ext_rs::warn!("unable to retrieve reverb track");
    false
}

pub fn update_player_preset(value: Preset) -> anyhow::Result<()> {
    if let Some(tracks) = TRACKS.get() {
        if let Ok(mut guard) = tracks.v.eq.try_lock() {
            guard.preset(value)?;
            red4ext_rs::info!("successfully updated player preset to {value}");
            return Ok(());
        }
        anyhow::bail!("lock contention")
    }
    anyhow::bail!("unable to reach tracks")
}
