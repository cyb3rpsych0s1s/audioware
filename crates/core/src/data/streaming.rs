use kira::{
    Tween,
    sound::streaming::{StreamingSoundData, StreamingSoundSettings},
};

use crate::{AudioDuration, With};

impl<T> AudioDuration for StreamingSoundData<T>
where
    T: Send + 'static,
{
    fn slice_duration(&self) -> std::time::Duration {
        self.duration()
    }

    fn total_duration(self) -> std::time::Duration {
        self.slice(None).loop_region(None).duration()
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
