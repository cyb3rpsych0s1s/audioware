use std::{collections::HashMap, sync::Mutex};

use audioware_sys::interop::{game::get_game_instance, quaternion::Quaternion, vector4::Vector4};
use glam::{Quat, Vec3};
use kira::{
    effect::{
        filter::{FilterBuilder, FilterHandle, FilterMode},
        reverb::ReverbBuilder,
    },
    spatial::{
        emitter::{EmitterHandle, EmitterSettings},
        listener::{ListenerHandle, ListenerSettings},
        scene::{SpatialSceneHandle, SpatialSceneSettings},
    },
    track::{TrackBuilder, TrackHandle, TrackRoutes},
    OutputDestination,
};
use once_cell::sync::OnceCell;
use red4ext_rs::types::{CName, EntityId};

use crate::types::error::TracksError;
use crate::types::{error::Error, id::SoundEntityId};

use super::{
    effects::{
        EqPass, HighPass, LowPass, Preset, EQ, EQ_HIGH_PASS_PHONE_CUTOFF, EQ_LOW_PASS_PHONE_CUTOFF,
        EQ_RESONANCE,
    },
    manager::audio_manager,
};

static TRACKS: OnceCell<Tracks> = OnceCell::new();
static SCENE: OnceCell<Scene> = OnceCell::new();

macro_rules! maybe_tracks {
    () => {
        TRACKS.get().ok_or(Error::from(TracksError::Uninitialized))
    };
}

macro_rules! maybe_eq {
    ($tracks:expr) => {
        $tracks
            .v
            .eq
            .try_lock()
            .map_err(|_| Error::from(TracksError::Uninitialized))
    };
}

#[allow(dead_code)]
struct Tracks {
    reverb: Mutex<TrackHandle>,
    v: V,
    holocall: Holocall,
}

#[allow(dead_code)]
struct V {
    main: Mutex<TrackHandle>,
    vocal: Mutex<TrackHandle>,
    mental: Mutex<TrackHandle>,
    emissive: Mutex<TrackHandle>,
    eq: Mutex<EQ>,
}

#[allow(dead_code)]
struct Holocall {
    main: TrackHandle,
    eq: Mutex<EQ>,
}

struct Scene {
    scene: Mutex<SpatialSceneHandle>,
    v: Mutex<ListenerHandle>,
    entities: Mutex<HashMap<SoundEntityId, EmitterHandle>>,
}

pub fn setup() -> anyhow::Result<()> {
    let mut manager = audio_manager().lock().unwrap();
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
            reverb: Mutex::new(reverb),
            v: V {
                main: Mutex::new(main),
                vocal: Mutex::new(vocal),
                mental: Mutex::new(mental),
                emissive: Mutex::new(emissive),
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
    SCENE
        .set(Scene {
            scene: Mutex::new(scene),
            v: Mutex::new(v),
            entities: Mutex::new(HashMap::new()),
        })
        .map_err(|_| anyhow::anyhow!("error setting audio engine spatial scene"))?;
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
            .and_then(|x| x.v.vocal.try_lock().ok())
            .map(|x| OutputDestination::from(&*x)),
        (Some(_), None, true, _) => TRACKS
            .get()
            .and_then(|x: &Tracks| x.v.emissive.try_lock().ok())
            .map(|x| OutputDestination::from(&*x)),
        (Some(id), _, false, _) => {
            red4ext_rs::info!(
                "retrieving entity id from scene ({})",
                u64::from(id.clone())
            );
            SCENE
                .get()
                .and_then(|x| x.entities.try_lock().ok())
                .and_then(|x| x.get(&SoundEntityId(id)).map(OutputDestination::from))
        }
        (None, Some(_), false, true) => TRACKS
            .get()
            .map(|x| &x.holocall.main)
            .map(OutputDestination::from),
        (None, _, _, _) => TRACKS
            .get()
            .and_then(|x| x.v.main.try_lock().ok())
            .map(|x| OutputDestination::from(&*x)),
    }
}

pub fn register_emitter(id: EntityId) {
    let key = SoundEntityId(id.clone());
    if let (Some(mut scene), Some(mut entities)) = (
        SCENE.get().and_then(|x| x.scene.try_lock().ok()),
        SCENE.get().and_then(|x| x.entities.try_lock().ok()),
    ) {
        if let std::collections::hash_map::Entry::Vacant(e) = entities.entry(key) {
            let entity =
                audioware_sys::interop::game::find_entity_by_id(get_game_instance(), id.clone());
            if let Some(entity) = entity.into_ref() {
                let position = entity.get_world_position();
                if let Ok(handle) = scene.add_emitter(position, EmitterSettings::default()) {
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
    if let Some(mut entities) = SCENE.get().and_then(|x| x.entities.try_lock().ok()) {
        if entities.contains_key(&key) {
            entities.remove(&key);
            red4ext_rs::info!("unregister emitter ({})", u64::from(id.clone()));
        }
    } else {
        red4ext_rs::error!("unable to get scene entities");
    }
}

pub fn update_listener(position: Vector4, orientation: Quaternion) {
    if let Some(mut listener) = SCENE.get().and_then(|x| x.v.try_lock().ok()) {
        listener.set_position(position.clone(), Default::default());
        listener.set_orientation(orientation.clone(), Default::default());
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
    if let Some(mut guard) = SCENE.get().and_then(|x| x.entities.try_lock().ok()) {
        if let Some(emitter) = guard.get_mut(&key) {
            emitter.set_position(position.clone(), Default::default());
        } else {
            red4ext_rs::error!("unable to get scene emitter ({})", u64::from(id.clone()));
        }
    } else {
        red4ext_rs::error!("unable to get scene entities");
    }
}

pub fn emitters_count() -> i32 {
    if let Some(guard) = SCENE.get().and_then(|x| x.entities.try_lock().ok()) {
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
        if let (Ok(reverb), Ok(mut main)) = (tracks.reverb.try_lock(), tracks.v.main.try_lock()) {
            if let Ok(()) = main.set_route(
                &*reverb,
                kira::Volume::Amplitude(value as f64),
                Default::default(),
            ) {
                return true;
            }
        }
        red4ext_rs::warn!("unable to update reverb route volume");
        return false;
    }
    red4ext_rs::warn!("unable to retrieve reverb track");
    false
}

pub fn update_player_preset(value: Preset) -> Result<(), Error> {
    let tracks = maybe_tracks!()?;
    let mut guard = maybe_eq!(tracks)?;
    guard.preset(value);
    red4ext_rs::info!("successfully updated player preset to {value}");
    Ok(())
}
