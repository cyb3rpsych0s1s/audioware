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
use red4ext_rs::types::{Ref, WeakRef};

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
    eq: DashMap<ControlId, EffectHandle<EqFilterHandle>>,
    distortions: DashMap<ControlId, EffectHandle<DistortionHandle>>,
    delays: DashMap<ControlId, EffectHandle<DelayHandle>>,
    compressors: DashMap<ControlId, EffectHandle<CompressorHandle>>,
    filters: DashMap<ControlId, EffectHandle<FilterHandle>>,
    reverbs: DashMap<ControlId, EffectHandle<ReverbHandle>>,
}

pub struct EffectHandle<T> {
    handle: T,
    owner: WeakRef<DynamicEffect>,
}

impl<T> Drop for EffectHandle<T> {
    fn drop(&mut self) {
        if let Some(mut owner) = self.owner.clone().upgrade()
            && let Some(owner) = unsafe { owner.fields_mut() }
            && owner.orphan.set(()).is_err()
        {
            fails!("effect handle use after-free");
        }
    }
}

impl Default for Effects {
    fn default() -> Self {
        Self {
            eq: DashMap::with_capacity(4),
            distortions: DashMap::with_capacity(4),
            delays: DashMap::with_capacity(4),
            compressors: DashMap::with_capacity(4),
            filters: DashMap::with_capacity(4),
            reverbs: DashMap::with_capacity(4),
        }
    }
}

impl Effects {
    pub fn insert(&mut self, builder: &mut SpatialTrackBuilder, effect: &mut Ref<DynamicEffect>) {
        let weak = effect.clone().downgrade();
        if let Some(effect) = unsafe { effect.fields_mut() } {
            let id = *effect.id.get().unwrap();
            if effect.as_ref().as_serializable().is_a::<DynamicEQ>() {
                self.insert_eq(
                    id,
                    builder,
                    unsafe { std::mem::transmute::<&DynamicEffect, &DynamicEQ>(effect) },
                    weak,
                );
            } else if effect
                .as_ref()
                .as_serializable()
                .is_a::<DynamicDistortion>()
            {
                self.insert_distortion(
                    id,
                    builder,
                    unsafe { std::mem::transmute::<&DynamicEffect, &DynamicDistortion>(effect) },
                    weak,
                );
            } else if effect.as_ref().as_serializable().is_a::<DynamicDelay>() {
                self.insert_delay(
                    id,
                    builder,
                    unsafe { std::mem::transmute::<&DynamicEffect, &DynamicDelay>(effect) },
                    weak,
                );
            } else if effect
                .as_ref()
                .as_serializable()
                .is_a::<DynamicCompressor>()
            {
                self.insert_compressor(
                    id,
                    builder,
                    unsafe { std::mem::transmute::<&DynamicEffect, &DynamicCompressor>(effect) },
                    weak,
                );
            } else {
                fails!(
                    "unknown dynamic effect class: {}",
                    effect.as_ref().class().name().as_str()
                )
            }
        }
    }
    fn insert_eq(
        &mut self,
        id: ControlId,
        builder: &mut SpatialTrackBuilder,
        effect: &DynamicEQ,
        owner: WeakRef<DynamicEffect>,
    ) {
        self.eq.insert(
            id,
            EffectHandle {
                handle: builder.add_effect(EqFilterBuilder::new(
                    effect.kind.get().into(),
                    effect.frequency.get() as f64,
                    effect.gain.get(),
                    effect.q.get() as f64,
                )),
                owner,
            },
        );
    }
    fn insert_distortion(
        &mut self,
        id: ControlId,
        builder: &mut SpatialTrackBuilder,
        effect: &DynamicDistortion,
        owner: WeakRef<DynamicEffect>,
    ) {
        self.distortions.insert(
            id,
            EffectHandle {
                handle: builder.add_effect(
                    DistortionBuilder::new()
                        .kind(effect.kind.get().into())
                        .drive(effect.drive.get())
                        .mix(effect.mix.get()),
                ),
                owner,
            },
        );
    }
    fn insert_delay(
        &mut self,
        id: ControlId,
        builder: &mut SpatialTrackBuilder,
        effect: &DynamicDelay,
        owner: WeakRef<DynamicEffect>,
    ) {
        self.delays.insert(
            id,
            EffectHandle {
                handle: builder.add_effect(
                    DelayBuilder::new()
                        .feedback(effect.feedback.get())
                        .delay_time(effect.delay_time.get())
                        .mix(effect.mix.get()),
                ),
                owner,
            },
        );
    }
    fn insert_compressor(
        &mut self,
        id: ControlId,
        builder: &mut SpatialTrackBuilder,
        effect: &DynamicCompressor,
        owner: WeakRef<DynamicEffect>,
    ) {
        self.compressors.insert(
            id,
            EffectHandle {
                handle: builder.add_effect(
                    CompressorBuilder::new()
                        .threshold(effect.threshold.get() as f64)
                        .ratio(effect.ratio.get() as f64)
                        .attack_duration(effect.attack_duration.get())
                        .release_duration(effect.release_duration.get())
                        .makeup_gain(effect.makeup_gain.get())
                        .mix(effect.mix.get()),
                ),
                owner,
            },
        );
    }
}

impl SetControlledEq for Effects {
    fn set_controlled_kind(&mut self, id: ControlId, kind: EqFilterKind) {
        if let Some(mut handle) = self.eq.get_mut(&id) {
            handle.handle.set_kind(kind);
        }
    }

    fn set_controlled_frequency(&mut self, id: ControlId, frequency: f64, tween: Tween) {
        if let Some(mut handle) = self.eq.get_mut(&id) {
            handle.handle.set_frequency(frequency, tween);
        }
    }

    fn set_controlled_gain(&mut self, id: ControlId, gain: audioware_core::Decibels, tween: Tween) {
        if let Some(mut handle) = self.eq.get_mut(&id) {
            handle.handle.set_gain(gain, tween);
        }
    }

    fn set_controlled_q(&mut self, id: ControlId, q: f64, tween: Tween) {
        if let Some(mut handle) = self.eq.get_mut(&id) {
            handle.handle.set_q(q, tween);
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
            handle.handle.set_kind(kind);
        }
    }

    fn set_controlled_drive(
        &mut self,
        id: ControlId,
        drive: audioware_core::Decibels,
        tween: Tween,
    ) {
        if let Some(mut handle) = self.distortions.get_mut(&id) {
            handle.handle.set_drive(drive, tween);
        }
    }

    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        if let Some(mut handle) = self.distortions.get_mut(&id) {
            handle.handle.set_mix(mix, tween);
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
            handle.handle.set_feedback(feedback, tween);
        }
    }

    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        if let Some(mut handle) = self.delays.get_mut(&id) {
            handle.handle.set_mix(mix, tween);
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
            handle.handle.set_threshold(threshold.as_f64(), tween);
        }
    }

    fn set_controlled_ratio(&mut self, id: ControlId, ratio: f32, tween: Tween) {
        if let Some(mut handle) = self.compressors.get_mut(&id) {
            handle.handle.set_ratio(ratio as f64, tween);
        }
    }

    fn set_controlled_attack_duration(
        &mut self,
        id: ControlId,
        attack_duration: std::time::Duration,
        tween: Tween,
    ) {
        if let Some(mut handle) = self.compressors.get_mut(&id) {
            handle.handle.set_attack_duration(attack_duration, tween);
        }
    }

    fn set_controlled_release_duration(
        &mut self,
        id: ControlId,
        release_duration: std::time::Duration,
        tween: Tween,
    ) {
        if let Some(mut handle) = self.compressors.get_mut(&id) {
            handle.handle.set_release_duration(release_duration, tween);
        }
    }

    fn set_controlled_makeup_gain(
        &mut self,
        id: ControlId,
        makeup_gain: audioware_core::Decibels,
        tween: Tween,
    ) {
        if let Some(mut handle) = self.compressors.get_mut(&id) {
            handle.handle.set_makeup_gain(makeup_gain, tween);
        }
    }

    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        if let Some(mut handle) = self.compressors.get_mut(&id) {
            handle.handle.set_mix(mix, tween);
        }
    }
}

impl SetControlledFilter for Effects {
    fn set_controlled_mode(&mut self, id: ControlId, mode: kira::effect::filter::FilterMode) {
        if let Some(mut handle) = self.filters.get_mut(&id) {
            handle.handle.set_mode(mode);
        }
    }

    fn set_controlled_cutoff(&mut self, id: ControlId, cutoff: f32, tween: Tween) {
        if let Some(mut handle) = self.filters.get_mut(&id) {
            handle.handle.set_cutoff(cutoff as f64, tween);
        }
    }

    fn set_controlled_resonance(&mut self, id: ControlId, resonance: f32, tween: Tween) {
        if let Some(mut handle) = self.filters.get_mut(&id) {
            handle.handle.set_resonance(resonance as f64, tween);
        }
    }

    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        if let Some(mut handle) = self.filters.get_mut(&id) {
            handle.handle.set_mix(mix, tween);
        }
    }
}

impl SetControlledReverb for Effects {
    fn set_controlled_feedback(&mut self, id: ControlId, feedback: f32, tween: Tween) {
        if let Some(mut handle) = self.reverbs.get_mut(&id) {
            handle.handle.set_feedback(feedback as f64, tween);
        }
    }

    fn set_controlled_damping(&mut self, id: ControlId, damping: f32, tween: Tween) {
        if let Some(mut handle) = self.reverbs.get_mut(&id) {
            handle.handle.set_damping(damping as f64, tween);
        }
    }

    fn set_controlled_stereo_width(&mut self, id: ControlId, stereo_width: f32, tween: Tween) {
        if let Some(mut handle) = self.reverbs.get_mut(&id) {
            handle.handle.set_stereo_width(stereo_width as f64, tween);
        }
    }

    fn set_controlled_mix(&mut self, id: ControlId, mix: f32, tween: Tween) {
        if let Some(mut handle) = self.reverbs.get_mut(&id) {
            handle.handle.set_mix(mix, tween);
        }
    }
}
