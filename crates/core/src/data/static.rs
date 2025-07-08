use kira::{
    Tween,
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};

use crate::{AudioDuration, SampleRate, With};

impl AudioDuration for StaticSoundData {
    fn slice_duration(&self) -> std::time::Duration {
        self.duration()
    }

    fn total_duration(self) -> std::time::Duration {
        self.slice(None).loop_region(None).duration()
    }
}

impl SampleRate for StaticSoundData {
    #[inline]
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

impl With<StaticSoundSettings> for StaticSoundData {
    #[inline]
    fn with(self, settings: StaticSoundSettings) -> Self
    where
        Self: Sized,
    {
        self.with_settings(settings)
    }
}

impl With<Tween> for StaticSoundData {
    #[inline]
    fn with(self, settings: Tween) -> Self
    where
        Self: Sized,
    {
        self.fade_in_tween(settings)
    }
}
