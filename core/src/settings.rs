//! Audio settings.

macro_rules! impl_audio_settings {
    ($ty:path) => {
        impl $crate::AudioSettings for $ty {
            #[inline]
            fn start_position(&self) -> ::kira::sound::PlaybackPosition {
                self.start_position.clone()
            }

            #[inline]
            fn region(&self) -> impl ::kira::sound::IntoOptionalRegion {
                self.loop_region
            }

            #[inline]
            fn r#loop(&self) -> bool {
                self.loop_region.is_some()
            }

            #[inline]
            fn volume(&self) -> ::kira::tween::Value<::kira::Volume> {
                self.volume.clone()
            }

            #[inline]
            fn playback_rate(&self) -> ::kira::tween::Value<::kira::sound::PlaybackRate> {
                self.playback_rate
            }

            #[inline]
            fn panning(&self) -> ::kira::tween::Value<f64> {
                self.panning
            }

            #[inline]
            fn start_time(&self) -> ::kira::StartTime {
                self.start_time
            }

            #[inline]
            fn output_destination(&self) -> ::kira::OutputDestination {
                self.output_destination
            }

            #[inline]
            fn fade_in_tween(&self) -> Option<::kira::tween::Tween> {
                self.fade_in_tween
            }
        }
    };
}

impl_audio_settings!(::kira::sound::static_sound::StaticSoundSettings);
impl_audio_settings!(::kira::sound::streaming::StreamingSoundSettings);
