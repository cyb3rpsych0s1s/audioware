use std::time::Duration;

use kira::StartTime;
use red4ext_rs::conv::NativeRepr;

#[derive(Debug, Default)]
#[repr(C)]
pub struct LinearTween {
    pub start_time: u32, // in milliseconds
    pub duration: u32,   // in milliseconds
}

unsafe impl NativeRepr for LinearTween {
    const NAME: &'static str = "Audioware.LinearTween";
}

impl From<LinearTween> for kira::tween::Tween {
    fn from(value: LinearTween) -> Self {
        let start_time = match value.start_time {
            0 => StartTime::Immediate,
            v => StartTime::Delayed(Duration::from_millis(v as u64)),
        };
        Self {
            start_time,
            duration: Duration::from_millis(value.duration as u64),
            easing: kira::tween::Easing::Linear,
        }
    }
}

#[derive(Debug, Default)]
#[repr(i64)]
pub enum Easing {
    #[default]
    InPowi = 0,
    OutPowi = 1,
    InOutPowi = 2,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ElasticTween {
    pub start_time: u32, // in milliseconds
    pub duration: u32,   // in milliseconds
    pub easing: Easing,
    /// easing intensity
    pub intensity: u32,
}

unsafe impl NativeRepr for ElasticTween {
    const NAME: &'static str = "Audioware.ElasticTween";
}

impl From<ElasticTween> for kira::tween::Tween {
    fn from(value: ElasticTween) -> Self {
        let start_time = match value.start_time {
            0 => StartTime::Immediate,
            v => StartTime::Delayed(Duration::from_millis(v as u64)),
        };
        let easing = match value.easing {
            Easing::InPowi => kira::tween::Easing::InPowi(value.intensity as i32),
            Easing::OutPowi => kira::tween::Easing::OutPowi(value.intensity as i32),
            Easing::InOutPowi => kira::tween::Easing::InOutPowi(value.intensity as i32),
        };
        Self {
            start_time,
            duration: Duration::from_millis(value.duration as u64),
            easing,
        }
    }
}
