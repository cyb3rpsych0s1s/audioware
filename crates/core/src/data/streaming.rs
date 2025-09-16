use kira::{
    Tween,
    sound::{
        PlaybackPosition,
        streaming::{StreamingSoundData, StreamingSoundSettings},
    },
};

use crate::{AudioDuration, With, settings::SceneDialogSettings};

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

impl<E: Send> With<SceneDialogSettings> for StreamingSoundData<E> {
    fn with(self, settings: SceneDialogSettings) -> Self
    where
        Self: Sized,
    {
        let given = self.settings.start_position;
        self.start_position(match given {
            PlaybackPosition::Seconds(x) => {
                PlaybackPosition::Seconds(x + settings.seek_time as f64)
            }
            PlaybackPosition::Samples(_) => {
                unreachable!("samples unit is not supported with streaming sound yet")
            }
        })
    }
}
