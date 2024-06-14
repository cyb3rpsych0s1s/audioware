use std::time::Duration;

use audioware_sys::interop::AsParent;
use kira::StartTime;
use red4ext_rs::{
    conv::{ClassType, NativeRepr},
    macros::redscript_import,
    types::{CName, IScriptable, Ref},
};

#[derive(Debug)]
pub struct AudiowareTween;

impl ClassType for AudiowareTween {
    type BaseClass = IScriptable;
    const NAME: &'static str = "AudiowareTween";
}

#[derive(Debug)]
pub struct AudiowareLinearTween;

impl ClassType for AudiowareLinearTween {
    type BaseClass = AudiowareTween;
    const NAME: &'static str = "AudiowareLinearTween";
}

#[redscript_import]
impl AudiowareLinearTween {
    pub fn start_time(self: &Ref<Self>) -> u32;
    pub fn duration(self: &Ref<Self>) -> u32;
}

#[derive(Debug)]
pub struct AudiowareElasticTween;

impl ClassType for AudiowareElasticTween {
    type BaseClass = AudiowareTween;
    const NAME: &'static str = "AudiowareElasticTween";
}

#[redscript_import]
impl AudiowareElasticTween {
    pub fn start_time(self: &Ref<Self>) -> u32;
    pub fn duration(self: &Ref<Self>) -> u32;
    pub fn easing(self: &Ref<Self>) -> AudiowareEasing;
    pub fn value(self: &Ref<Self>) -> i32;
}

#[derive(Debug, Default)]
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

pub trait IntoTween {
    fn into_tween(self) -> kira::tween::Tween;
}

impl IntoTween for Ref<AudiowareLinearTween> {
    fn into_tween(self) -> kira::tween::Tween {
        let start_time = match self.start_time() {
            0 => StartTime::Immediate,
            x => StartTime::Delayed(Duration::from_millis(x as u64)),
        };
        let duration = Duration::from_millis(self.duration() as u64);
        kira::tween::Tween {
            start_time,
            duration,
            easing: kira::tween::Easing::Linear,
        }
    }
}

impl IntoTween for Ref<AudiowareElasticTween> {
    fn into_tween(self) -> kira::tween::Tween {
        let start_time = match self.start_time() {
            0 => StartTime::Immediate,
            x => StartTime::Delayed(Duration::from_millis(x as u64)),
        };
        let duration = Duration::from_millis(self.duration() as u64);
        let easing = match self.easing() {
            AudiowareEasing::InPowi => kira::tween::Easing::InPowi(self.value()),
            AudiowareEasing::OutPowi => kira::tween::Easing::OutPowi(self.value()),
            AudiowareEasing::InOutPowi => kira::tween::Easing::InOutPowi(self.value()),
        };
        kira::tween::Tween {
            start_time,
            duration,
            easing,
        }
    }
}

pub trait AsChildTween {
    fn linear(&self) -> Option<Ref<AudiowareLinearTween>>;
    fn elastic(&self) -> Option<Ref<AudiowareElasticTween>>;
}

impl AsChildTween for Ref<AudiowareTween> {
    fn linear(&self) -> Option<Ref<AudiowareLinearTween>> {
        let base = self
            .as_parent()
            .expect("AudiowareTween extends IScriptable");
        if red4ext_rs::prelude::Ref::<red4ext_rs::prelude::IScriptable>::is_a(
            base,
            CName::new(AudiowareLinearTween::NATIVE_NAME),
        ) {
            let child = unsafe {
                std::mem::transmute::<
                    red4ext_rs::prelude::Ref<crate::types::AudiowareTween>,
                    red4ext_rs::prelude::Ref<crate::types::AudiowareLinearTween>,
                >(self.clone())
            };
            return Some(child);
        }
        None
    }

    fn elastic(&self) -> Option<Ref<AudiowareElasticTween>> {
        let base = self
            .as_parent()
            .expect("AudiowareTween extends IScriptable");
        if red4ext_rs::prelude::Ref::<red4ext_rs::prelude::IScriptable>::is_a(
            base,
            CName::new(AudiowareElasticTween::NATIVE_NAME),
        ) {
            let child = unsafe {
                std::mem::transmute::<
                    red4ext_rs::prelude::Ref<crate::types::AudiowareTween>,
                    red4ext_rs::prelude::Ref<crate::types::AudiowareElasticTween>,
                >(self.clone())
            };
            return Some(child);
        }
        None
    }
}
