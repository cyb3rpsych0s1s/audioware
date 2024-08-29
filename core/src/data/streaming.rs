use kira::{
    sound::streaming::{StreamingSoundData, StreamingSoundSettings},
    tween::Tween,
};

use crate::{AudioData, With};

impl<T> AudioData for StreamingSoundData<T>
where
    T: Send + 'static,
{
    type Settings = StreamingSoundSettings;

    #[inline]
    fn current_duration(&self) -> std::time::Duration {
        self.duration()
    }

    #[inline]
    fn total_duration(self) -> std::time::Duration {
        self.slice(None).loop_region(None).duration()
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

impl<T> With<Tween> for StreamingSoundData<T>
where
    T: Send + 'static,
{
    #[inline]
    fn with(self, settings: Tween) -> Self
    where
        Self: Sized,
    {
        self.fade_in_tween(settings)
    }
}
