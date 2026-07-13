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
            PlaybackPosition::Samples(samples) => {
                // The previous code called `unreachable!("samples unit is not
                // supported with streaming sound yet")`, which would crash the
                // engine thread (and under default `panic=unwind`, terminate
                // the audio engine silently) if a mod ever specified a
                // sample-based `start_position` on a streaming sound.
                //
                // We can't convert samples to seconds without the sample rate
                // (kira's StreamingSoundSettings does not expose it here), so
                // we fall back to ignoring the sample offset and apply only
                // the seek_time in seconds. This preserves the seek_time
                // behavior (used by `is_rewind` scene dialogs) while degrading
                // gracefully for the unsupported samples case.
                let _ = samples;
                PlaybackPosition::Seconds(settings.seek_time as f64)
            }
        })
    }
}
