use audioware_core::{Amplitude, SpatialTrackSettings, amplitude};
use kira::{
    AudioManager, Value,
    backend::Backend,
    effect::filter::{FilterBuilder, FilterHandle},
    listener::ListenerId,
    track::{SpatialTrackBuilder, SpatialTrackHandle},
};

use crate::{engine::tweens::OCCLUDED, error::Error};

use super::ambience::Ambience;

pub const DEFAULT_CUTOFF: f64 = 18_000.0;

pub struct Spatial {
    track: SpatialTrackHandle,
    occlusion: Option<FilterHandle>,
}

impl Spatial {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        listener: impl Into<ListenerId>,
        position: impl Into<Value<mint::Vector3<f32>>>,
        settings: SpatialTrackSettings,
        ambience: &Ambience,
    ) -> Result<Self, Error> {
        let SpatialTrackSettings {
            distances,
            persist_until_sounds_finish,
            attenuation_function,
            spatialization_strength,
            affected_by_reverb_mix,
            affected_by_environmental_preset,
            enable_occlusion,
        } = settings;
        let mut builder = SpatialTrackBuilder::new()
            .distances(distances)
            .spatialization_strength(spatialization_strength)
            .persist_until_sounds_finish(persist_until_sounds_finish)
            // None: disable volume attenuation based on distance
            .attenuation_function(attenuation_function.unwrap_or(kira::Easing::Linear));
        // sum used to have to be 1.0 otherwise sounds crackled, what now?
        if affected_by_reverb_mix {
            builder = builder.with_send(ambience.reverb(), amplitude!(0.5).as_decibels());
        }
        if affected_by_environmental_preset {
            builder = builder.with_send(ambience.environmental(), amplitude!(0.5).as_decibels());
        }
        let mut occlusion = None;
        if enable_occlusion {
            occlusion = Some(builder.add_effect(FilterBuilder::new().cutoff(DEFAULT_CUTOFF)));
        }
        let track = manager.add_spatial_sub_track(listener, position, builder)?;
        Ok(Self { track, occlusion })
    }
    pub fn set_occlusion(&mut self, factor: f32) {
        let normalized = (DEFAULT_CUTOFF * (1. - factor as f64)).clamp(600.0, DEFAULT_CUTOFF);
        if let Some(x) = self.occlusion.as_mut() {
            x.set_cutoff(normalized, OCCLUDED);
        }
    }
    pub fn occluded(&self) -> bool {
        self.occlusion.is_some()
    }
}

impl std::ops::Deref for Spatial {
    type Target = SpatialTrackHandle;

    fn deref(&self) -> &Self::Target {
        &self.track
    }
}

impl std::ops::DerefMut for Spatial {
    fn deref_mut(&mut self) -> &mut SpatialTrackHandle {
        &mut self.track
    }
}
