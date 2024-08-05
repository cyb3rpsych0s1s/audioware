use std::time::Duration;

use kira::tween::{Easing, Tween};
use red4ext_rs::{class_kind::Scripted, log, types::Ref, PluginOps, ScriptClass};

use crate::Audioware;

use super::AudiowareEasing;

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct AudiowareTween {
    /// delay before starting: in seconds
    start_time: f32,
    /// tween duration: in seconds
    duration: f32,
}
unsafe impl ScriptClass for AudiowareTween {
    type Kind = Scripted;
    const NAME: &'static str = "AudiowareTween";
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct AudiowareLinearTween {
    base: AudiowareTween,
}
unsafe impl ScriptClass for AudiowareLinearTween {
    type Kind = Scripted;
    const NAME: &'static str = "AudiowareLinearTween";
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct AudiowareElasticTween {
    base: AudiowareTween,
    /// tween curve
    easing: AudiowareEasing,
    /// tween curve intensity
    value: f32,
}
unsafe impl ScriptClass for AudiowareElasticTween {
    type Kind = Scripted;
    const NAME: &'static str = "AudiowareElasticTween";
}

pub trait ToTween {
    fn into_tween(self) -> Option<Tween>;
}

impl ToTween for Ref<AudiowareTween> {
    fn into_tween(self) -> Option<Tween> {
        if self.is_null() {
            return None;
        }
        if self.is_a::<AudiowareLinearTween>() {
            return self
                .clone()
                .cast::<AudiowareLinearTween>()
                .unwrap()
                .into_tween();
        }
        if self.is_a::<AudiowareElasticTween>() {
            return self
                .clone()
                .cast::<AudiowareElasticTween>()
                .unwrap()
                .into_tween();
        }
        None
    }
}

impl ToTween for Ref<AudiowareLinearTween> {
    fn into_tween(self) -> Option<Tween> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { self.fields() }.unwrap();
        let start_time = if value.base.start_time.is_finite() {
            value.base.start_time
        } else {
            log::error!(Audioware::env(), "start_time must be finite");
            0.
        };
        let duration = if value.base.duration.is_finite() {
            value.base.duration
        } else {
            log::error!(Audioware::env(), "duration must be finite");
            0.
        };
        Some(Tween {
            start_time: kira::StartTime::Delayed(Duration::from_secs_f32(start_time)),
            duration: Duration::from_secs_f32(duration),
            easing: Easing::Linear,
        })
    }
}

impl ToTween for Ref<AudiowareElasticTween> {
    fn into_tween(self) -> Option<Tween> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { self.fields() }.unwrap();
        let start_time = if value.base.start_time.is_finite() {
            value.base.start_time
        } else {
            log::error!(Audioware::env(), "start_time must be finite");
            0.
        };
        let duration = if value.base.duration.is_finite() {
            value.base.duration
        } else {
            log::error!(Audioware::env(), "duration must be finite");
            0.
        };
        let easing_value = if value.value.is_finite() {
            value.value
        } else {
            log::error!(Audioware::env(), "easing value must be finite");
            0.
        };
        Some(Tween {
            start_time: kira::StartTime::Delayed(Duration::from_secs_f32(start_time)),
            duration: Duration::from_secs_f32(duration),
            easing: match value.easing {
                AudiowareEasing::InPowf => Easing::InPowf(easing_value as f64),
                AudiowareEasing::OutPowf => Easing::OutPowf(easing_value as f64),
                AudiowareEasing::InOutPowf => Easing::InOutPowf(easing_value as f64),
            },
        })
    }
}

pub trait ToEasing {
    fn into_easing(self) -> Option<Easing>;
}

impl ToEasing for Ref<AudiowareLinearTween> {
    fn into_easing(self) -> Option<Easing> {
        if self.is_null() {
            return None;
        }
        Some(Easing::Linear)
    }
}

impl ToEasing for Ref<AudiowareElasticTween> {
    fn into_easing(self) -> Option<Easing> {
        if self.is_null() {
            return None;
        }
        let fields = unsafe { self.fields() }.unwrap();
        Some(match fields.easing {
            AudiowareEasing::InPowf => Easing::InPowf(fields.value as f64),
            AudiowareEasing::OutPowf => Easing::OutPowf(fields.value as f64),
            AudiowareEasing::InOutPowf => Easing::InOutPowf(fields.value as f64),
        })
    }
}

impl ToEasing for Ref<AudiowareTween> {
    fn into_easing(self) -> Option<Easing> {
        if self.is_null() {
            return None;
        }
        if self.is_a::<AudiowareLinearTween>() {
            return self
                .clone()
                .cast::<AudiowareLinearTween>()
                .unwrap()
                .into_easing();
        }
        if self.is_a::<AudiowareElasticTween>() {
            return self
                .clone()
                .cast::<AudiowareElasticTween>()
                .unwrap()
                .into_easing();
        }
        None
    }
}
