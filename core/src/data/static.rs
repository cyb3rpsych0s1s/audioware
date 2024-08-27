use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};

use crate::{AudioData, OptionalAudioSettings, SampleRate, With};

impl AudioData for StaticSoundData {
    type Settings = StaticSoundSettings;

    #[inline]
    fn duration(&self) -> std::time::Duration {
        self.duration()
    }

    #[inline]
    fn total_duration(self) -> std::time::Duration {
        self.loop_region(None).duration()
    }

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

impl<T: OptionalAudioSettings> With<T> for StaticSoundData {
    fn with(mut self, settings: T) -> Self
    where
        Self: Sized,
    {
        super::impl_with!(self, settings)
    }
}
