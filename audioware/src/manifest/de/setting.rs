use std::{marker::PhantomData, time::Duration};

use kira::{
    sound::{static_sound::StaticSoundSettings, streaming::StreamingSoundSettings},
    tween::Value,
    StartTime, Volume,
};
use serde::Deserialize;

use super::Usage;

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(with = "humantime_serde")]
    pub start_time: Option<Duration>,
    pub volume: Option<f64>,
    ghost: PhantomData<Usage>,
}

impl Clone for Settings {
    fn clone(&self) -> Self {
        Self {
            start_time: self.start_time,
            volume: self.volume,
            ghost: PhantomData,
        }
    }
}

impl From<Settings> for StaticSoundSettings {
    fn from(value: Settings) -> Self {
        Self {
            start_time: value
                .start_time
                .map(StartTime::Delayed)
                .unwrap_or(StartTime::Immediate),
            volume: value
                .volume
                .map(|x| Value::<Volume>::Fixed(Volume::Amplitude(x)))
                .unwrap_or(Value::<Volume>::Fixed(Volume::Amplitude(1.))),
            ..Default::default()
        }
    }
}

impl From<Settings> for StreamingSoundSettings {
    fn from(value: Settings) -> Self {
        Self {
            start_time: value
                .start_time
                .map(StartTime::Delayed)
                .unwrap_or(StartTime::Immediate),
            volume: value
                .volume
                .map(|x| Value::<Volume>::Fixed(Volume::Amplitude(x)))
                .unwrap_or(Value::<Volume>::Fixed(Volume::Amplitude(1.))),
            ..Default::default()
        }
    }
}
