use std::time::Duration;

use audioware_core::Decibels;
use kira::Tween;
use red4ext_rs::types::Ref;

use crate::{
    ControlId, ToTween,
    abi::{
        DistortionKind, DynamicCompressor, DynamicDelay, DynamicDistortion, DynamicEQ,
        DynamicEffect, DynamicFilter, DynamicReverb, EqFilterKind, FilterMode,
    },
    engine::queue,
    utils::warns,
};

pub enum DynamicEffectMsg {
    EQ(DynamicEQMsg),
    Distortion(DynamicDistortionMsg),
    Delay(DynamicDelayMsg),
    Compressor(DynamicCompressorMsg),
    Filter(DynamicFilterMsg),
    Reverb(DynamicReverbMsg),
}

#[allow(clippy::enum_variant_names)]
pub enum DynamicEQMsg {
    SetKind {
        id: ControlId,
        kind: kira::effect::eq_filter::EqFilterKind,
    },
    SetFrequency {
        id: ControlId,
        value: f32,
        tween: Option<Tween>,
    },
    SetGain {
        id: ControlId,
        value: Decibels,
        tween: Option<Tween>,
    },
    SetQ {
        id: ControlId,
        value: f32,
        tween: Option<Tween>,
    },
}

#[allow(clippy::enum_variant_names)]
pub enum DynamicDistortionMsg {
    SetKind {
        id: ControlId,
        kind: kira::effect::distortion::DistortionKind,
    },
    SetDrive {
        id: ControlId,
        value: Decibels,
        tween: Option<Tween>,
    },
    SetMix {
        id: ControlId,
        value: f32,
        tween: Option<Tween>,
    },
}

#[allow(clippy::enum_variant_names)]
pub enum DynamicDelayMsg {
    SetFeedback {
        id: ControlId,
        value: Decibels,
        tween: Option<Tween>,
    },
    SetMix {
        id: ControlId,
        value: f32,
        tween: Option<Tween>,
    },
}

#[allow(clippy::enum_variant_names)]
pub enum DynamicCompressorMsg {
    SetThreshold {
        id: ControlId,
        value: Decibels,
        tween: Option<Tween>,
    },
    SetRatio {
        id: ControlId,
        value: f32,
        tween: Option<Tween>,
    },
    SetAttackDuration {
        id: ControlId,
        value: Duration,
        tween: Option<Tween>,
    },
    SetReleaseDuration {
        id: ControlId,
        value: Duration,
        tween: Option<Tween>,
    },
    SetMakeupGain {
        id: ControlId,
        value: Decibels,
        tween: Option<Tween>,
    },
    SetMix {
        id: ControlId,
        value: f32,
        tween: Option<Tween>,
    },
}

#[allow(clippy::enum_variant_names)]
pub enum DynamicFilterMsg {
    SetMode {
        id: ControlId,
        value: kira::effect::filter::FilterMode,
    },
    SetCutoff {
        id: ControlId,
        value: f32,
        tween: Option<Tween>,
    },
    SetResonance {
        id: ControlId,
        value: f32,
        tween: Option<Tween>,
    },
    SetMix {
        id: ControlId,
        value: f32,
        tween: Option<Tween>,
    },
}

#[allow(clippy::enum_variant_names)]
pub enum DynamicReverbMsg {
    SetFeedback {
        id: ControlId,
        value: f32,
        tween: Option<Tween>,
    },
    SetDamping {
        id: ControlId,
        value: f32,
        tween: Option<Tween>,
    },
    SetStereoWidth {
        id: ControlId,
        value: f32,
        tween: Option<Tween>,
    },
    SetMix {
        id: ControlId,
        value: f32,
        tween: Option<Tween>,
    },
}

impl From<DynamicEQMsg> for DynamicEffectMsg {
    fn from(value: DynamicEQMsg) -> Self {
        Self::EQ(value)
    }
}

impl From<DynamicDistortionMsg> for DynamicEffectMsg {
    fn from(value: DynamicDistortionMsg) -> Self {
        Self::Distortion(value)
    }
}

impl From<DynamicDelayMsg> for DynamicEffectMsg {
    fn from(value: DynamicDelayMsg) -> Self {
        Self::Delay(value)
    }
}

impl From<DynamicCompressorMsg> for DynamicEffectMsg {
    fn from(value: DynamicCompressorMsg) -> Self {
        Self::Compressor(value)
    }
}

impl From<DynamicFilterMsg> for DynamicEffectMsg {
    fn from(value: DynamicFilterMsg) -> Self {
        Self::Filter(value)
    }
}

impl From<DynamicReverbMsg> for DynamicEffectMsg {
    fn from(value: DynamicReverbMsg) -> Self {
        Self::Reverb(value)
    }
}

impl DynamicEffect {
    pub fn is_active(&self) -> bool {
        self.id.get().is_some() && self.orphan.get().is_none()
    }
}

impl DynamicEQ {
    pub fn set_kind(&self, value: EqFilterKind) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("EQ hasn't been assigned to any emitter yet.");
            return;
        };
        queue::control_filter(DynamicEQMsg::SetKind {
            id,
            kind: value.into(),
        });
    }
    pub fn set_frequency(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("EQ hasn't been assigned to any emitter yet.");
            return;
        };
        queue::control_filter(DynamicEQMsg::SetFrequency {
            id,
            value,
            tween: tween.into_tween(),
        });
    }
    pub fn set_gain(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("EQ hasn't been assigned to any emitter yet.");
            return;
        };
        queue::control_filter(DynamicEQMsg::SetGain {
            id,
            value: Decibels::from(value),
            tween: tween.into_tween(),
        });
    }
    pub fn set_q(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("EQ hasn't been assigned to any emitter yet.");
            return;
        };
        if value <= 0.0 {
            warns!("frequency range width (a.k.a 'Q'): cannot be lower or equals to 0.");
            return;
        }
        queue::control_filter(DynamicEQMsg::SetQ {
            id,
            value,
            tween: tween.into_tween(),
        });
    }
}

impl DynamicDistortion {
    pub fn set_kind(&self, value: DistortionKind) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Distortion hasn't been assigned to any emitter yet.");
            return;
        };
        queue::control_filter(DynamicDistortionMsg::SetKind {
            id,
            kind: value.into(),
        });
    }
    pub fn set_drive(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Distortion hasn't been assigned to any emitter yet.");
            return;
        };
        queue::control_filter(DynamicDistortionMsg::SetDrive {
            id,
            value: Decibels::from(value),
            tween: tween.into_tween(),
        });
    }
    pub fn set_mix(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Distortion hasn't been assigned to any emitter yet.");
            return;
        };
        if !(0.0..=1.0).contains(&value) {
            warns!("invalid mix ({value}): must be between 0.0 and 1.0.");
            return;
        }
        queue::control_filter(DynamicDistortionMsg::SetMix {
            id,
            value,
            tween: tween.into_tween(),
        });
    }
}

impl DynamicDelay {
    pub fn set_feedback(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Delay hasn't been assigned to any emitter yet.");
            return;
        };
        queue::control_filter(DynamicDelayMsg::SetFeedback {
            id,
            value: Decibels::from(value),
            tween: tween.into_tween(),
        });
    }
    pub fn set_mix(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Delay hasn't been assigned to any emitter yet.");
            return;
        };
        if !(0.0..=1.0).contains(&value) {
            warns!("invalid mix ({value}): must be between 0.0 and 1.0.");
            return;
        }
        queue::control_filter(DynamicDelayMsg::SetMix {
            id,
            value,
            tween: tween.into_tween(),
        });
    }
}

impl DynamicCompressor {
    pub fn set_threshold(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Compressor hasn't been assigned to any emitter yet.");
            return;
        };
        queue::control_filter(DynamicCompressorMsg::SetThreshold {
            id,
            value: Decibels::from(value),
            tween: tween.into_tween(),
        });
    }
    pub fn set_ratio(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Compressor hasn't been assigned to any emitter yet.");
            return;
        };
        if value < 0. {
            warns!("invalid ratio ({value}): cannot be lower than 0.");
            return;
        }
        queue::control_filter(DynamicCompressorMsg::SetRatio {
            id,
            value,
            tween: tween.into_tween(),
        });
    }
    pub fn set_attack_duration(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Compressor hasn't been assigned to any emitter yet.");
            return;
        };
        let Ok(value) = Duration::try_from_secs_f32(value).inspect_err(|e| {
            warns!("invalid attack duration ({value}): {e}");
        }) else {
            return;
        };
        queue::control_filter(DynamicCompressorMsg::SetAttackDuration {
            id,
            value,
            tween: tween.into_tween(),
        });
    }
    pub fn set_release_duration(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Compressor hasn't been assigned to any emitter yet.");
            return;
        };
        let Ok(value) = Duration::try_from_secs_f32(value).inspect_err(|e| {
            warns!("invalid release duration ({value}): {e}");
        }) else {
            return;
        };
        queue::control_filter(DynamicCompressorMsg::SetReleaseDuration {
            id,
            value,
            tween: tween.into_tween(),
        });
    }
    pub fn set_makeup_gain(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Compressor hasn't been assigned to any emitter yet.");
            return;
        };
        queue::control_filter(DynamicCompressorMsg::SetMakeupGain {
            id,
            value: Decibels::from(value),
            tween: tween.into_tween(),
        });
    }
    pub fn set_mix(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Compressor hasn't been assigned to any emitter yet.");
            return;
        };
        if !(0.0..=1.0).contains(&value) {
            warns!("invalid mix ({value}): must be between 0.0 and 1.0.");
            return;
        }
        queue::control_filter(DynamicCompressorMsg::SetMix {
            id,
            value,
            tween: tween.into_tween(),
        });
    }
}

impl DynamicFilter {
    pub fn set_mode(&self, value: FilterMode) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Filter hasn't been assigned to any emitter yet.");
            return;
        };
        queue::control_filter(DynamicFilterMsg::SetMode {
            id,
            value: value.into(),
        });
    }
    pub fn set_cutoff(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Filter hasn't been assigned to any emitter yet.");
            return;
        };
        queue::control_filter(DynamicFilterMsg::SetCutoff {
            id,
            value,
            tween: tween.into_tween(),
        });
    }
    pub fn set_resonance(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Filter hasn't been assigned to any emitter yet.");
            return;
        };
        queue::control_filter(DynamicFilterMsg::SetResonance {
            id,
            value,
            tween: tween.into_tween(),
        });
    }
    pub fn set_mix(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Filter hasn't been assigned to any emitter yet.");
            return;
        };
        if !(0.0..=1.0).contains(&value) {
            warns!("invalid mix ({value}): must be between 0.0 and 1.0.");
            return;
        }
        queue::control_filter(DynamicFilterMsg::SetMix {
            id,
            value,
            tween: tween.into_tween(),
        });
    }
}

impl DynamicReverb {
    pub fn set_feedback(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Reverb hasn't been assigned to any emitter yet.");
            return;
        };
        if !(0.0..=1.0).contains(&value) {
            warns!("invalid feedback ({value}): must be between 0.0 and 1.0.");
            return;
        }
        queue::control_filter(DynamicReverbMsg::SetFeedback {
            id,
            value,
            tween: tween.into_tween(),
        });
    }
    pub fn set_damping(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Reverb hasn't been assigned to any emitter yet.");
            return;
        };
        queue::control_filter(DynamicReverbMsg::SetDamping {
            id,
            value,
            tween: tween.into_tween(),
        });
    }
    pub fn set_stereo_width(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Reverb hasn't been assigned to any emitter yet.");
            return;
        };
        if !(0.0..=1.0).contains(&value) {
            warns!("invalid stereo width ({value}): must be between 0.0 and 1.0.");
            return;
        }
        queue::control_filter(DynamicReverbMsg::SetStereoWidth {
            id,
            value,
            tween: tween.into_tween(),
        });
    }
    pub fn set_mix(&self, value: f32, tween: Ref<crate::Tween>) {
        let Some(id) = self.base.id.get().cloned() else {
            warns!("Reverb hasn't been assigned to any emitter yet.");
            return;
        };
        if !(0.0..=1.0).contains(&value) {
            warns!("invalid mix ({value}): must be between 0.0 and 1.0.");
            return;
        }
        queue::control_filter(DynamicReverbMsg::SetMix {
            id,
            value,
            tween: tween.into_tween(),
        });
    }
}

impl std::fmt::Display for DynamicEffectMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::EQ(x) => format!("eq: {x}"),
                Self::Distortion(x) => format!("distortion: {x}"),
                Self::Delay(x) => format!("delay: {x}"),
                Self::Compressor(x) => format!("compressor: {x}"),
                Self::Filter(x) => format!("filter: {x}"),
                Self::Reverb(x) => format!("reverb: {x}"),
            }
        )
    }
}

impl std::fmt::Display for DynamicEQMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::SetKind { id, kind, .. } => format!(
                    "set kind: {} ({id})",
                    match kind {
                        kira::effect::eq_filter::EqFilterKind::Bell => "bell",
                        kira::effect::eq_filter::EqFilterKind::LowShelf => "low shelf",
                        kira::effect::eq_filter::EqFilterKind::HighShelf => "high shelf",
                    }
                ),
                Self::SetFrequency { id, value, .. } => format!("set frequency: {value} ({id})"),
                Self::SetGain { id, value, .. } => format!("set gain: {value} ({id})"),
                Self::SetQ { id, value, .. } => format!("set q: {value} ({id})"),
            }
        )
    }
}

impl std::fmt::Display for DynamicDistortionMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::SetKind { id, kind, .. } => format!(
                    "set kind: {} ({id})",
                    match kind {
                        kira::effect::distortion::DistortionKind::HardClip => "hard clip",
                        kira::effect::distortion::DistortionKind::SoftClip => "soft clip",
                    }
                ),
                Self::SetDrive { id, value, .. } => format!("set drive: {value} ({id})"),
                Self::SetMix { id, value, .. } => format!("set mix: {value} ({id})"),
            }
        )
    }
}

impl std::fmt::Display for DynamicDelayMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::SetFeedback { id, value, .. } => format!("set feedback: {value} ({id})"),
                Self::SetMix { id, value, .. } => format!("set mix: {value} ({id})"),
            }
        )
    }
}

impl std::fmt::Display for DynamicCompressorMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::SetThreshold { id, value, .. } => format!("set threshold: {value} ({id})"),
                Self::SetRatio { id, value, .. } => format!("set ratio: {value} ({id})"),
                Self::SetAttackDuration { id, value, .. } =>
                    format!("set attack duration: {}s ({id})", value.as_secs_f32()),
                Self::SetReleaseDuration { id, value, .. } =>
                    format!("set release duration: {}s ({id})", value.as_secs_f32()),
                Self::SetMakeupGain { id, value, .. } => format!("set makeup gain: {value} ({id})"),
                Self::SetMix { id, value, .. } => format!("set mix: {value} ({id})"),
            }
        )
    }
}

impl std::fmt::Display for DynamicFilterMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::SetMode { id, value } => format!(
                    "set mode: {} ({id})",
                    match value {
                        kira::effect::filter::FilterMode::LowPass => "lowpass",
                        kira::effect::filter::FilterMode::BandPass => "bandpass",
                        kira::effect::filter::FilterMode::HighPass => "highpass",
                        kira::effect::filter::FilterMode::Notch => "notch",
                    }
                ),
                Self::SetCutoff { id, value, .. } => format!("set cutoff: {value} ({id})"),
                Self::SetResonance { id, value, .. } => format!("set resonance: {value} ({id})"),
                Self::SetMix { id, value, .. } => format!("set mix: {value} ({id})"),
            }
        )
    }
}

impl std::fmt::Display for DynamicReverbMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::SetFeedback { id, value, .. } => format!("set feedback: {value} ({id})"),
                Self::SetDamping { id, value, .. } => format!("set damping: {value} ({id})"),
                Self::SetStereoWidth { id, value, .. } =>
                    format!("set stereo width: {value} ({id})"),
                Self::SetMix { id, value, .. } => format!("set mix: {value} ({id})"),
            }
        )
    }
}
