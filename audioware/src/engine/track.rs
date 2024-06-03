use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use glam::{Quat, Vec3};
use kira::{
    effect::{
        filter::{FilterBuilder, FilterHandle, FilterMode},
        reverb::ReverbBuilder,
    },
    spatial::{
        emitter::EmitterHandle,
        listener::{ListenerHandle, ListenerSettings},
        scene::{SpatialSceneHandle, SpatialSceneSettings},
    },
    track::{TrackBuilder, TrackHandle, TrackRoutes},
};
use once_cell::sync::OnceCell;
use snafu::OptionExt;

use crate::error::UninitializedSnafu;

use super::{
    effect::{
        HighPass, LowPass, EQ, EQ_HIGH_PASS_PHONE_CUTOFF, EQ_LOW_PASS_PHONE_CUTOFF, EQ_RESONANCE,
    },
    error::Error,
    id::SoundEntityId,
    manager::audio_manager,
};

pub static TRACKS: OnceCell<Tracks> = OnceCell::new();
pub static SCENE: OnceCell<Scene> = OnceCell::new();

#[inline(always)]
pub fn maybe_tracks<'cell>() -> Result<&'cell Tracks, Error> {
    Ok(TRACKS
        .get()
        .context(UninitializedSnafu { which: "tracks" })?)
}

#[inline(always)]
pub fn maybe_scene<'cell>() -> Result<&'cell Scene, Error> {
    Ok(SCENE.get().context(UninitializedSnafu { which: "scene" })?)
}

#[inline(always)]
pub fn maybe_equalizer<'guard>() -> Result<MutexGuard<'guard, EQ>, Error> {
    maybe_tracks()?
        .v
        .eq
        .try_lock()
        .map_err(|e| crate::error::Error::from(e).into())
}

pub struct Tracks {
    pub reverb: TrackHandle,
    pub v: V,
    pub holocall: Holocall,
}

pub struct V {
    pub main: TrackHandle,
    pub vocal: TrackHandle,
    pub mental: TrackHandle,
    pub emissive: TrackHandle,
    pub eq: Mutex<EQ>,
}

pub struct Holocall {
    pub main: TrackHandle,
    pub eq: Mutex<EQ>,
}

pub struct Scene {
    pub scene: Arc<Mutex<SpatialSceneHandle>>,
    pub v: Arc<Mutex<ListenerHandle>>,
    pub entities: Arc<Mutex<HashMap<SoundEntityId, EmitterHandle>>>,
}

impl Tracks {
    pub fn try_new() -> Result<(), Error> {
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
        let vocal =
            manager.add_sub_track(TrackBuilder::new().routes(TrackRoutes::parent(&main)))?;
        let mental =
            manager.add_sub_track(TrackBuilder::new().routes(TrackRoutes::parent(&main)))?;
        let emissive =
            manager.add_sub_track(TrackBuilder::new().routes(TrackRoutes::parent(&main)))?;
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
            .map_err(|_| Error::Internal {
                source: crate::error::Error::CannotSet { which: "tracks" },
            })?;
        SCENE
            .set(Scene {
                scene: Arc::new(Mutex::new(scene)),
                v: Arc::new(Mutex::new(v)),
                entities: Arc::new(Mutex::new(HashMap::new())),
            })
            .map_err(|_| Error::Internal {
                source: crate::error::Error::CannotSet { which: "scene" },
            })?;
        Ok(())
    }
}
