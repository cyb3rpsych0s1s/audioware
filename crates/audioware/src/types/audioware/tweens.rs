use std::{hash::Hash, time::Duration};

use red4ext_rs::{class_kind::Scripted, log, types::Ref, PluginOps, ScriptClass};

use crate::Audioware;

use super::Easing;

/// Intermediate representation for [kira::tween::Tween].
#[derive(Debug, PartialEq, Clone, Default)]
#[repr(C)]
pub struct Tween {
    /// delay before starting: in seconds
    pub start_time: f32,
    /// tween duration: in seconds
    pub duration: f32,
}
unsafe impl ScriptClass for Tween {
    type Kind = Scripted;
    const NAME: &'static str = "Audioware.Tween";
}
impl Hash for Tween {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.start_time as u64).hash(state);
        (self.duration as u64).hash(state);
    }
}

/// Intermediate representation for [kira::tween::Tween]
/// used in Redscript.
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct LinearTween {
    base: Tween,
}
unsafe impl ScriptClass for LinearTween {
    type Kind = Scripted;
    const NAME: &'static str = "Audioware.LinearTween";
}

impl LinearTween {
    pub fn start_time(&self) -> f32 {
        self.base.start_time
    }
    pub fn duration(&self) -> f32 {
        self.base.duration
    }
}

impl Hash for LinearTween {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.base.hash(state);
        Self::NAME.hash(state);
    }
}

/// Intermediate representation for [kira::tween::Tween]
/// used in Redscript.
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct ElasticTween {
    base: Tween,
    /// tween curve
    pub easing: Easing,
    /// tween curve intensity
    pub value: f32,
}
unsafe impl ScriptClass for ElasticTween {
    type Kind = Scripted;
    const NAME: &'static str = "Audioware.ElasticTween";
}
impl Hash for ElasticTween {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.base.hash(state);
        self.easing.hash(state);
        ((self.value * 100.) as u64).hash(state);
        Self::NAME.hash(state);
    }
}

impl ElasticTween {
    pub fn start_time(&self) -> f32 {
        self.base.start_time
    }
    pub fn duration(&self) -> f32 {
        self.base.duration
    }
}

/// Any type that can converted into an optional [kira::tween::Tween].
pub trait ToTween {
    fn into_tween(self) -> Option<kira::tween::Tween>;
}

impl ToTween for kira::tween::Tween {
    fn into_tween(self) -> Option<kira::tween::Tween> {
        Some(self)
    }
}

impl<T> ToTween for Option<T>
where
    T: ToTween,
{
    fn into_tween(self) -> Option<kira::tween::Tween> {
        self.and_then(ToTween::into_tween)
    }
}

impl ToTween for Ref<Tween> {
    fn into_tween(self) -> Option<kira::tween::Tween> {
        if self.is_null() {
            return None;
        }
        if self.is_a::<LinearTween>() {
            return self.clone().cast::<LinearTween>().unwrap().into_tween();
        }
        if self.is_a::<ElasticTween>() {
            return self.clone().cast::<ElasticTween>().unwrap().into_tween();
        }
        None
    }
}

impl ToTween for Ref<LinearTween> {
    fn into_tween(self) -> Option<kira::tween::Tween> {
        if self.is_null() {
            return None;
        }
        let fields = unsafe { self.fields() }.unwrap();
        let start_time = if fields.base.start_time.is_finite() {
            fields.base.start_time
        } else {
            log::error!(Audioware::env(), "start_time must be finite");
            0.
        };
        let duration = if fields.base.duration.is_finite() {
            fields.base.duration
        } else {
            log::error!(Audioware::env(), "duration must be finite");
            0.
        };
        Some(kira::tween::Tween {
            start_time: kira::StartTime::Delayed(Duration::from_secs_f32(start_time)),
            duration: Duration::from_secs_f32(duration),
            easing: kira::tween::Easing::Linear,
        })
    }
}

impl ToTween for Ref<ElasticTween> {
    fn into_tween(self) -> Option<kira::tween::Tween> {
        if self.is_null() {
            return None;
        }
        let fields = unsafe { self.fields() }.unwrap();
        let start_time = if fields.base.start_time.is_finite() {
            fields.base.start_time
        } else {
            log::error!(Audioware::env(), "start_time must be finite");
            0.
        };
        let duration = if fields.base.duration.is_finite() {
            fields.base.duration
        } else {
            log::error!(Audioware::env(), "duration must be finite");
            0.
        };
        let easing_value = if fields.value.is_finite() {
            fields.value
        } else {
            log::error!(Audioware::env(), "easing value must be finite");
            0.
        };
        Some(kira::tween::Tween {
            start_time: kira::StartTime::Delayed(Duration::from_secs_f32(start_time)),
            duration: Duration::from_secs_f32(duration),
            easing: match fields.easing {
                Easing::InPowf => kira::tween::Easing::InPowf(easing_value as f64),
                Easing::OutPowf => kira::tween::Easing::OutPowf(easing_value as f64),
                Easing::InOutPowf => kira::tween::Easing::InOutPowf(easing_value as f64),
            },
        })
    }
}

/// Any type that can be converted into an optional [kira::tween::Easing].
pub trait ToEasing {
    fn into_easing(self) -> Option<kira::tween::Easing>;
}

impl ToEasing for Ref<LinearTween> {
    fn into_easing(self) -> Option<kira::tween::Easing> {
        if self.is_null() {
            return None;
        }
        Some(kira::tween::Easing::Linear)
    }
}

impl ToEasing for Ref<ElasticTween> {
    fn into_easing(self) -> Option<kira::tween::Easing> {
        if self.is_null() {
            return None;
        }
        let fields = unsafe { self.fields() }.unwrap();
        Some(match fields.easing {
            Easing::InPowf => kira::tween::Easing::InPowf(fields.value as f64),
            Easing::OutPowf => kira::tween::Easing::OutPowf(fields.value as f64),
            Easing::InOutPowf => kira::tween::Easing::InOutPowf(fields.value as f64),
        })
    }
}

impl ToEasing for Ref<Tween> {
    fn into_easing(self) -> Option<kira::tween::Easing> {
        if self.is_null() {
            return None;
        }
        if self.is_a::<LinearTween>() {
            return self.clone().cast::<LinearTween>().unwrap().into_easing();
        }
        if self.is_a::<ElasticTween>() {
            return self.clone().cast::<ElasticTween>().unwrap().into_easing();
        }
        None
    }
}
