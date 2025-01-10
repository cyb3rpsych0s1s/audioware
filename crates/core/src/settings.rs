//! Audio settings.

use kira::{track::SpatialTrackDistances, Easing, Mix};

#[derive(Debug)]
pub struct SpatialTrackSettings {
    pub distances: SpatialTrackDistances,
    pub persist_until_sounds_finish: bool,
    pub attenuation_function: Option<Easing>,
    pub spatialization_strength: Mix,
}

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
            fn volume(&self) -> ::kira::Value<::kira::Decibels> {
                self.volume.clone()
            }

            #[inline]
            fn playback_rate(&self) -> ::kira::Value<::kira::PlaybackRate> {
                self.playback_rate
            }

            #[inline]
            fn panning(&self) -> ::kira::Value<::kira::Panning> {
                self.panning
            }

            #[inline]
            fn start_time(&self) -> ::kira::StartTime {
                self.start_time
            }

            #[inline]
            fn fade_in_tween(&self) -> Option<::kira::Tween> {
                self.fade_in_tween
            }
        }
    };
}

impl_audio_settings!(::kira::sound::static_sound::StaticSoundSettings);
impl_audio_settings!(::kira::sound::streaming::StreamingSoundSettings);
