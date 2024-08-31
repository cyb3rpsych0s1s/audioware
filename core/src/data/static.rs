use kira::{
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    tween::Tween,
};

use crate::{AudioData, AudioDuration, SampleRate, With};

impl AudioData for StaticSoundData {
    type Settings = StaticSoundSettings;

    #[inline]
    fn settings(&self) -> &Self::Settings {
        &self.settings
    }

    #[inline]
    fn slice(&self) -> Option<(usize, usize)> {
        self.slice
    }

    #[inline]
    fn with_slice(self, region: impl kira::sound::IntoOptionalRegion) -> Self {
        self.slice(region)
    }
}

impl AudioDuration for StaticSoundData {
    fn slice_duration(&self) -> std::time::Duration {
        self.duration()
    }

    fn loop_duration(self) -> Option<std::time::Duration> {
        self.settings
            .loop_region
            .map(|x| x.duration(self.frames.len(), self.sample_rate))
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
