use std::time::Duration;

use kira::tween::{Easing, Tween};
use red4ext_rs::{
    class_kind::Scripted,
    types::{IScriptable, Ref},
    NativeRepr, ScriptClass,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i64)]
pub enum AudiowareEasing {
    #[default]
    InPowi = 0,
    OutPowi = 1,
    InOutPowi = 2,
}
unsafe impl NativeRepr for AudiowareEasing {
    const NAME: &'static str = "AudiowareEasing";
}

#[repr(C)]
pub struct AudiowareTween {
    base: IScriptable,
    /// delay before starting: in milliseconds
    start_time: u32,
    /// tween duration: in milliseconds
    duration: u32,
}
unsafe impl ScriptClass for AudiowareTween {
    type Kind = Scripted;
    const NAME: &'static str = "AudiowareTween";
}

#[repr(C)]
pub struct AudiowareLinearTween {
    base: AudiowareTween,
}
unsafe impl ScriptClass for AudiowareLinearTween {
    type Kind = Scripted;
    const NAME: &'static str = "AudiowareLinearTween";
}

#[repr(C)]
pub struct AudiowareElasticTween {
    base: AudiowareTween,
    /// tween curve
    easing: AudiowareEasing,
    /// tween curve intensity
    value: i32,
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
            return Some(
                unsafe { self.cast::<AudiowareLinearTween>().unwrap().fields() }
                    .unwrap()
                    .into(),
            );
        }
        if self.is_a::<AudiowareElasticTween>() {
            return Some(
                unsafe { self.cast::<AudiowareElasticTween>().unwrap().fields() }
                    .unwrap()
                    .into(),
            );
        }
        None
    }
}

impl<'a> From<&'a AudiowareLinearTween> for Tween {
    fn from(value: &'a AudiowareLinearTween) -> Self {
        Self {
            start_time: kira::StartTime::Delayed(Duration::from_millis(
                value.base.start_time as u64,
            )),
            duration: Duration::from_millis(value.base.start_time as u64),
            easing: Easing::Linear,
        }
    }
}

impl<'a> From<&'a AudiowareElasticTween> for Tween {
    fn from(value: &'a AudiowareElasticTween) -> Self {
        Self {
            start_time: kira::StartTime::Delayed(Duration::from_millis(
                value.base.start_time as u64,
            )),
            duration: Duration::from_millis(value.base.start_time as u64),
            easing: match value.easing {
                AudiowareEasing::InPowi => Easing::InPowi(value.value),
                AudiowareEasing::OutPowi => Easing::OutPowi(value.value),
                AudiowareEasing::InOutPowi => Easing::InOutPowi(value.value),
            },
        }
    }
}
