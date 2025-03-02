//! Audio settings.

use kira::{
    track::{SpatialTrackBuilder, SpatialTrackDistances},
    Easing,
};

#[derive(Debug, Clone)]
pub struct SpatialTrackSettings {
    pub distances: SpatialTrackDistances,
    pub persist_until_sounds_finish: bool,
    pub attenuation_function: Option<Easing>,
    pub spatialization_strength: f32,
    pub affected_by_reverb_mix: bool,
    pub affected_by_environmental_preset: bool,
}

impl Default for SpatialTrackSettings {
    fn default() -> Self {
        Self {
            distances: SpatialTrackDistances::default(),
            persist_until_sounds_finish: false,
            attenuation_function: None,
            spatialization_strength: 0.75,
            affected_by_reverb_mix: true,
            affected_by_environmental_preset: false,
        }
    }
}

impl From<SpatialTrackSettings> for SpatialTrackBuilder {
    fn from(value: SpatialTrackSettings) -> Self {
        Self::new()
            .distances(value.distances)
            .persist_until_sounds_finish(value.persist_until_sounds_finish)
            .spatialization_strength(value.spatialization_strength)
            .attenuation_function(value.attenuation_function)
    }
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
