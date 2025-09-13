use kira::{
    Tween,
    sound::{
        PlaybackPosition,
        static_sound::{StaticSoundData, StaticSoundSettings},
    },
};

use crate::{AudioDuration, SampleRate, With, settings::SceneDialogSettings};

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

impl With<SceneDialogSettings> for StaticSoundData {
    fn with(self, settings: SceneDialogSettings) -> Self
    where
        Self: Sized,
    {
        if settings.is_rewind {
            return self.start_position(match self.settings.start_position {
                PlaybackPosition::Seconds(x) => {
                    PlaybackPosition::Seconds(x + settings.seek_time as f64)
                }
                PlaybackPosition::Samples(x) => PlaybackPosition::Samples(
                    x + (settings.seek_time * self.sample_rate as f32) as usize,
                ),
            });
        }
        self
    }
}
