use audioware_core::{Amplitude, SpatialTrackSettings, amplitude};
use debug_ignore::DebugIgnore;
use kira::{
    AudioManager, Value,
    backend::Backend,
    effect::filter::{FilterBuilder, FilterHandle},
    listener::ListenerId,
    track::{SpatialTrackBuilder, SpatialTrackHandle},
};
use red4ext_rs::types::Ref;

use crate::{
    abi::DynamicEffect,
    engine::{
        effects::Effects,
        traits::{
            compressor::SetControlledCompressor, delay::SetControlledDelay,
            distortion::SetControlledDistortion, eq::SetControlledEq, filter::SetControlledFilter,
            reverb::SetControlledReverb,
        },
        tweens::OCCLUDED,
    },
    error::Error,
};

use super::ambience::Ambience;

pub const DEFAULT_CUTOFF: f64 = 18_000.0;

pub struct Spatial {
    track: SpatialTrackHandle,
    occlusion: Option<FilterHandle>,
    effects: Effects,
}

impl Spatial {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        listener: impl Into<ListenerId>,
        position: impl Into<Value<mint::Vector3<f32>>>,
        settings: SpatialTrackSettings,
        effects: &mut [DebugIgnore<Ref<DynamicEffect>>],
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
        let mut stored = Effects::default();
        for effect in effects.iter_mut().filter(|x| !x.is_null()) {
            stored.insert(&mut builder, effect);
        }
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
        Ok(Self {
            track,
            occlusion,
            effects: stored,
        })
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

impl SetControlledEq for Spatial {
    #[inline]
    fn set_controlled_kind(
        &mut self,
        id: crate::ControlId,
        kind: kira::effect::eq_filter::EqFilterKind,
    ) {
        SetControlledEq::set_controlled_kind(&mut self.effects, id, kind);
    }

    #[inline]
    fn set_controlled_frequency(
        &mut self,
        id: crate::ControlId,
        frequency: f64,
        tween: kira::Tween,
    ) {
        self.effects.set_controlled_frequency(id, frequency, tween);
    }

    #[inline]
    fn set_controlled_gain(
        &mut self,
        id: crate::ControlId,
        gain: audioware_core::Decibels,
        tween: kira::Tween,
    ) {
        self.effects.set_controlled_gain(id, gain, tween);
    }

    #[inline]
    fn set_controlled_q(&mut self, id: crate::ControlId, q: f64, tween: kira::Tween) {
        self.effects.set_controlled_q(id, q, tween);
    }
}

impl SetControlledDistortion for Spatial {
    #[inline]
    fn set_controlled_kind(
        &mut self,
        id: crate::ControlId,
        kind: kira::effect::distortion::DistortionKind,
    ) {
        SetControlledDistortion::set_controlled_kind(&mut self.effects, id, kind);
    }

    #[inline]
    fn set_controlled_drive(
        &mut self,
        id: crate::ControlId,
        drive: audioware_core::Decibels,
        tween: kira::Tween,
    ) {
        self.effects.set_controlled_drive(id, drive, tween);
    }

    #[inline]
    fn set_controlled_mix(&mut self, id: crate::ControlId, mix: f32, tween: kira::Tween) {
        SetControlledDistortion::set_controlled_mix(&mut self.effects, id, mix, tween);
    }
}

impl SetControlledDelay for Spatial {
    #[inline]
    fn set_controlled_feedback(
        &mut self,
        id: crate::ControlId,
        feedback: audioware_core::Decibels,
        tween: kira::Tween,
    ) {
        SetControlledDelay::set_controlled_feedback(&mut self.effects, id, feedback, tween);
    }

    #[inline]
    fn set_controlled_mix(&mut self, id: crate::ControlId, mix: f32, tween: kira::Tween) {
        SetControlledDelay::set_controlled_mix(&mut self.effects, id, mix, tween);
    }
}

impl SetControlledCompressor for Spatial {
    #[inline]
    fn set_controlled_threshold(
        &mut self,
        id: crate::ControlId,
        threshold: audioware_core::Decibels,
        tween: kira::Tween,
    ) {
        self.effects.set_controlled_threshold(id, threshold, tween);
    }

    #[inline]
    fn set_controlled_ratio(&mut self, id: crate::ControlId, ratio: f32, tween: kira::Tween) {
        self.effects.set_controlled_ratio(id, ratio, tween);
    }

    #[inline]
    fn set_controlled_attack_duration(
        &mut self,
        id: crate::ControlId,
        attack_duration: std::time::Duration,
        tween: kira::Tween,
    ) {
        self.effects
            .set_controlled_attack_duration(id, attack_duration, tween);
    }

    #[inline]
    fn set_controlled_release_duration(
        &mut self,
        id: crate::ControlId,
        release_duration: std::time::Duration,
        tween: kira::Tween,
    ) {
        self.effects
            .set_controlled_release_duration(id, release_duration, tween);
    }

    #[inline]
    fn set_controlled_makeup_gain(
        &mut self,
        id: crate::ControlId,
        makeup_gain: audioware_core::Decibels,
        tween: kira::Tween,
    ) {
        self.effects
            .set_controlled_makeup_gain(id, makeup_gain, tween);
    }

    #[inline]
    fn set_controlled_mix(&mut self, id: crate::ControlId, mix: f32, tween: kira::Tween) {
        SetControlledCompressor::set_controlled_mix(&mut self.effects, id, mix, tween);
    }
}

impl SetControlledFilter for Spatial {
    #[inline]
    fn set_controlled_mode(
        &mut self,
        id: crate::ControlId,
        mode: kira::effect::filter::FilterMode,
    ) {
        self.effects.set_controlled_mode(id, mode);
    }

    #[inline]
    fn set_controlled_cutoff(&mut self, id: crate::ControlId, cutoff: f32, tween: kira::Tween) {
        self.effects.set_controlled_cutoff(id, cutoff, tween);
    }

    #[inline]
    fn set_controlled_resonance(
        &mut self,
        id: crate::ControlId,
        resonance: f32,
        tween: kira::Tween,
    ) {
        self.effects.set_controlled_resonance(id, resonance, tween);
    }

    #[inline]
    fn set_controlled_mix(&mut self, id: crate::ControlId, mix: f32, tween: kira::Tween) {
        SetControlledFilter::set_controlled_mix(&mut self.effects, id, mix, tween);
    }
}

impl SetControlledReverb for Spatial {
    #[inline]
    fn set_controlled_feedback(&mut self, id: crate::ControlId, feedback: f32, tween: kira::Tween) {
        SetControlledReverb::set_controlled_feedback(&mut self.effects, id, feedback, tween);
    }

    #[inline]
    fn set_controlled_damping(&mut self, id: crate::ControlId, damping: f32, tween: kira::Tween) {
        self.effects.set_controlled_damping(id, damping, tween);
    }

    #[inline]
    fn set_controlled_stereo_width(
        &mut self,
        id: crate::ControlId,
        stereo_width: f32,
        tween: kira::Tween,
    ) {
        self.effects
            .set_controlled_stereo_width(id, stereo_width, tween);
    }

    #[inline]
    fn set_controlled_mix(&mut self, id: crate::ControlId, mix: f32, tween: kira::Tween) {
        SetControlledReverb::set_controlled_mix(&mut self.effects, id, mix, tween);
    }
}
