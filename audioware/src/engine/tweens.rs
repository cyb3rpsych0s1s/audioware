use std::time::Duration;

use kira::{
    tween::{Easing, Tween},
    StartTime,
};
use red4ext_rs::types::CName;

use crate::ToTween;

pub const IMMEDIATELY: Tween = Tween {
    start_time: StartTime::Immediate,
    duration: Duration::ZERO,
    easing: Easing::Linear,
};

pub const LAST_BREATH: Tween = Tween {
    start_time: StartTime::Immediate,
    duration: Duration::from_millis(2300),
    easing: Easing::InPowf(0.6),
};

pub const DEFAULT: Tween = Tween {
    start_time: StartTime::Immediate,
    duration: Duration::from_millis(10),
    easing: Easing::Linear,
};

pub const DILATION_LINEAR: Tween = Tween {
    start_time: StartTime::Immediate,
    duration: Duration::from_millis(400),
    easing: Easing::Linear,
};

pub const DILATION_EASE_IN: Tween = Tween {
    start_time: StartTime::Immediate,
    duration: Duration::from_millis(400),
    easing: Easing::OutPowf(0.6),
};

pub const DILATION_EASE_OUT: Tween = Tween {
    start_time: StartTime::Immediate,
    duration: Duration::from_secs(2),
    easing: Easing::InPowf(3.),
};

impl ToTween for CName {
    fn into_tween(self) -> Option<Tween> {
        match self.as_str() {
            "Linear" => Some(DILATION_LINEAR),
            "slowMoEaseIn"
            | "MeleeHitEaseIn"
            | "KereznikovSlideEaseIn"
            | "KereznikovDodgeEaseIn" => Some(DILATION_EASE_IN),
            "requestKerenzikovDeactivationWithEaseOut"
            | "slowMoEaseOut"
            | "MeleeHitEaseOut"
            | "DiveEaseOut"
            | "KereznikovDodgeEaseOut"
            | "KerenzikovEaseOut"
            | "SandevistanEaseOut" => Some(DILATION_EASE_OUT),
            _ => None,
        }
    }
}
