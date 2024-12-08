use std::time::Duration;

use audioware_manifest::{Interpolation, Locale, LocaleExt, PlayerGender, Region, Settings};
use kira::{manager::backend::cpal::CpalBackend, tween::Easing};
use red4ext_rs::{
    class_kind::{Native, Scripted},
    types::{CName, IScriptable, Opt, Ref, StaticArray},
    ScriptClass,
};

use crate::{
    abi::fails, engine::Engine, ElasticTween, EmitterDistances, EmitterSettings, LinearTween,
    ToEasing, Tween, AUDIOWARE_VERSION,
};

/// Represents a region in time.
/// Useful to describe a portion of a sound.
#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct AudioRegion {
    starts: f32,
    ends: f32,
}

unsafe impl ScriptClass for AudioRegion {
    type Kind = Scripted;
    const NAME: &'static str = "Audioware.AudioRegion";
}

/// Extended audio settings.
#[derive(Clone)]
#[repr(C)]
pub struct AudioSettingsExt {
    start_position: f32,
    region: Ref<AudioRegion>,
    r#loop: bool,
    volume: f32,
    fade_in: Ref<Tween>,
    panning: f32,
    playback_rate: f32,
    affected_by_time_dilation: bool,
}

impl Default for AudioSettingsExt {
    fn default() -> Self {
        Self {
            start_position: 0.,
            region: Ref::default(),
            r#loop: false,
            volume: 1.,
            fade_in: Ref::default(),
            panning: 0.5,
            playback_rate: 1.,
            affected_by_time_dilation: true,
        }
    }
}

unsafe impl ScriptClass for AudioSettingsExt {
    type Kind = Scripted;
    const NAME: &'static str = "Audioware.AudioSettingsExt";
}

impl std::fmt::Debug for AudioSettingsExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioSettingsExt")
            .field("start_position", &self.start_position)
            .field("loop", &self.r#loop)
            .field("volume", &self.volume)
            .field("panning", &self.panning)
            .field("playback_rate", &self.playback_rate)
            .finish_non_exhaustive()
    }
}

pub trait ToSettings {
    type Settings;
    fn into_settings(self) -> Option<Self::Settings>;
}

pub trait ToRegion {
    fn into_region(self) -> Option<Region>;
}

pub trait ToInterpolation {
    fn into_interpolation(self) -> Option<Interpolation>;
}

impl ToInterpolation for Ref<Tween> {
    fn into_interpolation(self) -> Option<Interpolation> {
        if self.is_null() {
            return None;
        }
        match self.clone() {
            x if x.is_a::<LinearTween>() => {
                let x = x.cast::<LinearTween>().unwrap();
                let x = unsafe { x.fields() }?;
                Some(Interpolation {
                    start_time: x
                        .start_time()
                        .ne(&0.)
                        .then_some(Duration::from_secs_f32(x.start_time())),
                    duration: Duration::from_secs_f32(x.duration()),
                    easing: Easing::Linear,
                })
            }
            x if x.is_a::<ElasticTween>() => {
                let x = x.cast::<ElasticTween>().unwrap();
                let x = unsafe { x.fields() }?;
                Some(Interpolation {
                    start_time: x
                        .start_time()
                        .ne(&0.)
                        .then_some(Duration::from_secs_f32(x.start_time())),
                    duration: Duration::from_secs_f32(x.duration()),
                    easing: match x.easing {
                        crate::Easing::InPowf => Easing::InPowf(x.value as f64),
                        crate::Easing::OutPowf => Easing::OutPowf(x.value as f64),
                        crate::Easing::InOutPowf => Easing::InOutPowf(x.value as f64),
                    },
                })
            }
            _ => unreachable!(),
        }
    }
}

impl ToRegion for Ref<AudioRegion> {
    fn into_region(self) -> Option<Region> {
        if self.is_null() {
            return None;
        }
        let AudioRegion { starts, ends } = unsafe { self.fields() }?.clone();
        if starts == 0. && ends == 0. {
            return None;
        }
        Some(Region {
            starts: starts.ne(&0.).then_some(Duration::from_secs_f32(starts)),
            ends: ends.ne(&0.).then_some(Duration::from_secs_f32(ends)),
        })
    }
}

impl ToSettings for Ref<AudioSettingsExt> {
    type Settings = Settings;
    fn into_settings(self) -> Option<Self::Settings> {
        if self.is_null() {
            return None;
        }
        let AudioSettingsExt {
            start_position,
            region,
            r#loop,
            volume,
            fade_in,
            panning,
            playback_rate,
            affected_by_time_dilation,
        } = unsafe { self.fields() }?.clone();
        if let Err(e) = Duration::try_from_secs_f32(start_position) {
            fails!("invalid start position: {e}");
            return None;
        }
        Some(Settings {
            start_time: Default::default(),
            start_position: Some(Duration::from_secs_f32(start_position)),
            region: region.into_region(),
            r#loop: Some(r#loop),
            volume: Some(volume as f64),
            fade_in_tween: fade_in.into_interpolation(),
            panning: Some(panning as f64),
            playback_rate: Some(kira::sound::PlaybackRate::Factor(playback_rate as f64)),
            affected_by_time_dilation: Some(affected_by_time_dilation),
        })
    }
}

impl ToSettings for Ref<EmitterSettings> {
    type Settings = kira::spatial::emitter::EmitterSettings;
    fn into_settings(self) -> Option<Self::Settings> {
        if self.is_null() {
            return None;
        }
        let EmitterSettings {
            distances,
            attenuation_function,
            enable_spatialization,
            persist_until_sounds_finish,
        } = unsafe { self.fields() }?.clone();
        Some(kira::spatial::emitter::EmitterSettings {
            distances: distances.into_settings().unwrap_or_default(),
            attenuation_function: attenuation_function.into_easing(),
            enable_spatialization,
            persist_until_sounds_finish,
        })
    }
}

impl ToSettings for Ref<EmitterDistances> {
    type Settings = kira::spatial::emitter::EmitterDistances;
    fn into_settings(self) -> Option<Self::Settings> {
        if self.is_null() {
            return None;
        }
        let EmitterDistances {
            min_distance,
            max_distance,
        } = unsafe { self.fields() }?.clone();
        Some(kira::spatial::emitter::EmitterDistances {
            min_distance,
            max_distance,
        })
    }
}

/// Interop type for [Ext.reds](https://github.com/cyb3rpsych0s1s/audioware/blob/main/audioware/reds/Ext.reds).
#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct AudioSystemExt {
    base: IScriptable,
}

impl AudioSystemExt {
    pub const fn is_debug(&self) -> bool {
        cfg!(debug_assertions)
    }
    pub fn duration(
        &self,
        event_name: CName,
        locale: Opt<LocaleExt>,
        gender: Opt<PlayerGender>,
        total: Opt<bool>,
    ) -> f32 {
        let locale = match locale.into_option().map(Locale::try_from) {
            None => None,
            Some(Ok(x)) => Some(x),
            Some(Err(x)) => {
                fails!("invalid locale ({x})");
                return -1.;
            }
        };
        Engine::<CpalBackend>::duration(
            event_name,
            locale.unwrap_or_default(),
            gender.into_option().unwrap_or_default(),
            total.into_option().unwrap_or_default(),
        )
    }
    pub fn semantic_version(&self) -> StaticArray<u16, 5> {
        StaticArray::from(AUDIOWARE_VERSION)
    }
}

unsafe impl ScriptClass for AudioSystemExt {
    type Kind = Native;
    const NAME: &'static str = "AudioSystemExt";
}
