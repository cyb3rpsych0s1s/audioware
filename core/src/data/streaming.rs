use kira::sound::streaming::{StreamingSoundData, StreamingSoundSettings};

use crate::{AudioData, OptionalAudioSettings, With};

impl<T> AudioData for StreamingSoundData<T>
where
    T: Send + 'static,
{
    type Settings = StreamingSoundSettings;

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

impl<T> With<StreamingSoundSettings> for StreamingSoundData<T>
where
    T: Send + 'static,
{
    #[inline]
    fn with(self, settings: StreamingSoundSettings) -> Self
    where
        Self: Sized,
    {
        self.with_settings(settings)
    }
}

impl<T, U: OptionalAudioSettings> With<U> for StreamingSoundData<T>
where
    T: Send + 'static,
{
    fn with(mut self, settings: U) -> Self
    where
        Self: Sized,
    {
        super::impl_with!(self, settings)
    }
}
