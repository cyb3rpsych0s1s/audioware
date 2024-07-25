use std::time::Duration;

use kira::{
    tween::{Easing, Tween},
    StartTime,
};

pub const SMOOTHLY: Tween = Tween {
    start_time: StartTime::Immediate,
    duration: Duration::from_millis(200),
    easing: Easing::Linear,
};

pub const IMMEDIATELY: Tween = Tween {
    start_time: StartTime::Immediate,
    duration: Duration::ZERO,
    easing: Easing::Linear,
};

pub const DEFAULT: Tween = Tween {
    start_time: StartTime::Immediate,
    duration: Duration::from_millis(10),
    easing: Easing::Linear,
};
