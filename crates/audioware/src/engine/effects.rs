use dashmap::DashMap;
use kira::{
    Tween,
    effect::{
        compressor::{CompressorBuilder, CompressorHandle},
        delay::{DelayBuilder, DelayHandle},
        distortion::{DistortionBuilder, DistortionHandle},
        eq_filter::{EqFilterBuilder, EqFilterHandle, EqFilterKind},
        filter::FilterHandle,
        reverb::ReverbHandle,
    },
    track::SpatialTrackBuilder,
};

use crate::{
    ControlId,
    abi::{DynamicCompressor, DynamicDelay, DynamicDistortion, DynamicEQ, DynamicEffect},
    engine::traits::{
        compressor::SetControlledCompressor, delay::SetControlledDelay,
        distortion::SetControlledDistortion, eq::SetControlledEq, filter::SetControlledFilter,
        reverb::SetControlledReverb,
    },
    utils::fails,
};

pub struct Effects {
    eq: DashMap<ControlId, EqFilterHandle>,
    distortions: DashMap<ControlId, DistortionHandle>,
    delays: DashMap<ControlId, DelayHandle>,
    compressors: DashMap<ControlId, CompressorHandle>,
    filters: DashMap<ControlId, FilterHandle>,
    reverbs: DashMap<ControlId, ReverbHandle>,
}

impl Default for Effects {
    fn default() -> Self {
        Self {
            eq: DashMap::with_capacity(8),
            distortions: DashMap::with_capacity(8),
            delays: DashMap::with_capacity(8),
            compressors: DashMap::with_capacity(8),
            filters: DashMap::with_capacity(8),
            reverbs: DashMap::with_capacity(8),
        }
    }
}

impl Effects {
    pub fn insert(
        &mut self,
        id: ControlId,
        builder: &mut SpatialTrackBuilder,
        effect: &DynamicEffect,
    ) {
        if effect.as_ref().as_serializable().is_a::<DynamicEQ>() {
            self.insert_eq(id, builder, unsafe {
                std::mem::transmute::<&DynamicEffect, &DynamicEQ>(effect)
            });
        } else if effect
            .as_ref()
            .as_serializable()
            .is_a::<DynamicDistortion>()
        {
            self.insert_distortion(id, builder, unsafe {
                std::mem::transmute::<&DynamicEffect, &DynamicDistortion>(effect)
            });
        } else if effect.as_ref().as_serializable().is_a::<DynamicDelay>() {
            self.insert_delay(id, builder, unsafe {
                std::mem::transmute::<&DynamicEffect, &DynamicDelay>(effect)
            });
        } else if effect
            .as_ref()
            .as_serializable()
            .is_a::<DynamicCompressor>()
        {
            self.insert_compressor(id, builder, unsafe {
                std::mem::transmute::<&DynamicEffect, &DynamicCompressor>(effect)
            });
        } else {
            fails!(
                "unknown dynamic effect class: {}",
                effect.as_ref().class().name().as_str()
            )
        }
    }
    fn insert_eq(&mut self, id: ControlId, builder: &mut SpatialTrackBuilder, effect: &DynamicEQ) {
        self.eq.insert(
            id,
            builder.add_effect(EqFilterBuilder::new(
                effect.kind.get().into(),
                effect.frequency.get() as f64,
                effect.gain.get(),
                effect.q.get() as f64,
            )),
        );
    }
    fn insert_distortion(
        &mut self,
        id: ControlId,
        builder: &mut SpatialTrackBuilder,
        effect: &DynamicDistortion,
    ) {
        self.distortions.insert(
            id,
            builder.add_effect(
                DistortionBuilder::new()
                    .kind(effect.kind.get().into())
                    .drive(effect.drive.get())
                    .mix(effect.mix.get()),
            ),
        );
    }
    fn insert_delay(
        &mut self,
        id: ControlId,
        builder: &mut SpatialTrackBuilder,
        effect: &DynamicDelay,
    ) {
        self.delays.insert(
            id,
            builder.add_effect(
                DelayBuilder::new()
                    .feedback(effect.feedback.get())
                    .mix(effect.mix.get()),
            ),
        );
    }
    fn insert_compressor(
        &mut self,
        id: ControlId,
        builder: &mut SpatialTrackBuilder,
        effect: &DynamicCompressor,
    ) {
        self.compressors.insert(
            id,
            builder.add_effect(
                CompressorBuilder::new()
                    .threshold(effect.threshold.get() as f64)
                    .ratio(effect.ratio.get() as f64)
                    .attack_duration(effect.attack_duration.get())
                    .release_duration(effect.release_duration.get())
                    .makeup_gain(effect.makeup_gain.get())
                    .mix(effect.mix.get()),
            ),
        );
    }
}

impl SetControlledEq for Effects {
    fn set_controlled_kind(&mut self, id: ControlId, kind: EqFilterKind) {
        if let Some(mut handle) = self.eq.get_mut(&id) {
            handle.set_kind(kind);
        }
    }

    fn set_controlled_frequency(&mut self, id: ControlId, frequency: f64, tween: Tween) {
        if let Some(mut handle) = self.eq.get_mut(&id) {
            handle.set_frequency(frequency, tween);
        }
    }

    fn set_controlled_gain(&mut self, id: ControlId, gain: audioware_core::Decibels, tween: Tween) {
        if let Some(mut handle) = self.eq.get_mut(&id) {
            handle.set_gain(gain, tween);
        }
    }

    fn set_controlled_q(&mut self, id: ControlId, q: f64, tween: Tween) {
        if let Some(mut handle) = self.eq.get_mut(&id) {
            handle.set_q(q, tween);
        }
    }
}

impl SetControlledDistortion for Effects {
    fn set_controlled_kind(
        &mut self,
        id: ControlId,
        kind: kira::effect::distortion::DistortionKind,
    ) {
        if let Some(mut handle) = self.distortions.get_mut(&id) {
            handle.set_kind(kind);
        }
    }

    fn set_controlled_drive(
        &mut self,
        id: ControlId,
        drive: audioware_core::Decibels,
        tween: Tween,
    ) {
        if let Some(mut handle) = self.distortions.get_mut(&id) {
            handle.set_drive(drive, tween);
        }
    }

    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        if let Some(mut handle) = self.distortions.get_mut(&id) {
            handle.set_mix(mix, tween);
        }
    }
}

impl SetControlledDelay for Effects {
    fn set_controlled_feedback(
        &mut self,
        id: ControlId,
        feedback: audioware_core::Decibels,
        tween: Tween,
    ) {
        if let Some(mut handle) = self.delays.get_mut(&id) {
            handle.set_feedback(feedback, tween);
        }
    }

    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        if let Some(mut handle) = self.delays.get_mut(&id) {
            handle.set_mix(mix, tween);
        }
    }
}

impl SetControlledCompressor for Effects {
    fn set_controlled_threshold(
        &mut self,
        id: ControlId,
        threshold: audioware_core::Decibels,
        tween: Tween,
    ) {
        if let Some(mut handle) = self.compressors.get_mut(&id) {
            handle.set_threshold(threshold.as_f64(), tween);
        }
    }

    fn set_controlled_ratio(&mut self, id: ControlId, ratio: f32, tween: Tween) {
        if let Some(mut handle) = self.compressors.get_mut(&id) {
            handle.set_ratio(ratio as f64, tween);
        }
    }

    fn set_controlled_attack_duration(
        &mut self,
        id: ControlId,
        attack_duration: std::time::Duration,
        tween: Tween,
    ) {
        if let Some(mut handle) = self.compressors.get_mut(&id) {
            handle.set_attack_duration(attack_duration, tween);
        }
    }

    fn set_controlled_release_duration(
        &mut self,
        id: ControlId,
        release_duration: std::time::Duration,
        tween: Tween,
    ) {
        if let Some(mut handle) = self.compressors.get_mut(&id) {
            handle.set_release_duration(release_duration, tween);
        }
    }

    fn set_controlled_makeup_gain(
        &mut self,
        id: ControlId,
        makeup_gain: audioware_core::Decibels,
        tween: Tween,
    ) {
        if let Some(mut handle) = self.compressors.get_mut(&id) {
            handle.set_makeup_gain(makeup_gain, tween);
        }
    }

    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        if let Some(mut handle) = self.compressors.get_mut(&id) {
            handle.set_mix(mix, tween);
        }
    }
}

impl SetControlledFilter for Effects {
    fn set_controlled_mode(&mut self, id: ControlId, mode: kira::effect::filter::FilterMode) {
        if let Some(mut handle) = self.filters.get_mut(&id) {
            handle.set_mode(mode);
        }
    }

    fn set_controlled_cutoff(&mut self, id: ControlId, cutoff: f32, tween: Tween) {
        if let Some(mut handle) = self.filters.get_mut(&id) {
            handle.set_cutoff(cutoff as f64, tween);
        }
    }

    fn set_controlled_resonance(&mut self, id: ControlId, resonance: f32, tween: Tween) {
        if let Some(mut handle) = self.filters.get_mut(&id) {
            handle.set_resonance(resonance as f64, tween);
        }
    }

    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        if let Some(mut handle) = self.filters.get_mut(&id) {
            handle.set_mix(mix, tween);
        }
    }
}

impl SetControlledReverb for Effects {
    fn set_controlled_feedback(&mut self, id: ControlId, feedback: f32, tween: Tween) {
        if let Some(mut handle) = self.reverbs.get_mut(&id) {
            handle.set_feedback(feedback as f64, tween);
        }
    }

    fn set_controlled_damping(&mut self, id: ControlId, damping: f32, tween: Tween) {
        if let Some(mut handle) = self.reverbs.get_mut(&id) {
            handle.set_damping(damping as f64, tween);
        }
    }

    fn set_controlled_stereo_width(&mut self, id: ControlId, stereo_width: f32, tween: Tween) {
        if let Some(mut handle) = self.reverbs.get_mut(&id) {
            handle.set_stereo_width(stereo_width as f64, tween);
        }
    }

    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        if let Some(mut handle) = self.reverbs.get_mut(&id) {
            handle.set_mix(mix, tween);
        }
    }
}
