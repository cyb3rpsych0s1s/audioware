use std::{
    cell::{Cell, RefCell},
    hash::{Hash, Hasher},
    num::NonZero,
    ops::{Deref, Not},
    sync::OnceLock,
    time::Duration,
};

use audioware_core::{Amplitude, SpatialTrackSettings};
use audioware_manifest::{Interpolation, Locale, LocaleExt, PlayerGender, Region, Settings};
use kira::{Easing, backend::cpal::CpalBackend, track::SpatialTrackDistances};
use red4ext_rs::{
    ScriptClass,
    class_kind::{Native, Scripted},
    types::{CName, EntityId, GameInstance, IScriptable, Opt, Ref, StaticArray},
};

use crate::{
    AUDIOWARE_VERSION, AsEntity, ControlId, ElasticTween, EmitterDistances, EmitterSettings,
    Entity, EventName, LinearTween, ToEasing, Tween, abi::fails, engine::Engine,
    error::ValidationError, get_player,
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

impl<T> ToSettings for Ref<T>
where
    T: ScriptClass + Clone + ToSettings,
    Ref<T>: Clone,
{
    type Settings = <T as ToSettings>::Settings;

    fn into_settings(self) -> Option<Self::Settings> {
        if self.is_null() {
            return None;
        }
        unsafe { self.fields() }
            .cloned()
            .and_then(ToSettings::into_settings)
    }
}

pub trait ToSettingsExt {
    type Settings;
    type Defaults;
    fn into_settings_ext(self, defaults: Self::Defaults) -> Option<Self::Settings>;
}

impl<T> ToSettingsExt for Ref<T>
where
    T: ScriptClass + Clone + ToSettingsExt,
    Ref<T>: Clone,
{
    type Settings = <T as ToSettingsExt>::Settings;
    type Defaults = <T as ToSettingsExt>::Defaults;

    fn into_settings_ext(self, defaults: Self::Defaults) -> Option<Self::Settings> {
        if self.is_null() {
            return None;
        }
        unsafe { self.fields() }
            .cloned()
            .and_then(|x| x.into_settings_ext(defaults))
    }
}

pub trait ToRegion {
    fn into_region(self) -> Option<Region>;
}

impl<T> ToRegion for Ref<T>
where
    T: ScriptClass + Clone + ToRegion,
    Ref<T>: Clone,
{
    fn into_region(self) -> Option<Region> {
        if self.is_null() {
            return None;
        }
        unsafe { self.fields() }
            .cloned()
            .and_then(ToRegion::into_region)
    }
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
                        .then(|| Duration::from_secs_f32(x.start_time())),
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
                        .then(|| Duration::from_secs_f32(x.start_time())),
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

impl ToRegion for AudioRegion {
    fn into_region(self) -> Option<Region> {
        Some(Region {
            starts: self
                .starts
                .ne(&0.)
                .then(|| Duration::from_secs_f32(self.starts)),
            ends: self
                .ends
                .ne(&0.)
                .then(|| Duration::from_secs_f32(self.ends)),
        })
    }
}

impl ToSettings for AudioSettingsExt {
    type Settings = Settings;
    fn into_settings(self) -> Option<Self::Settings> {
        if let Err(e) = Duration::try_from_secs_f32(self.start_position) {
            fails!("invalid start position: {e}");
            return None;
        }
        let Ok(volume) = Amplitude::try_from(self.volume) else {
            fails!("invalid volume ({})", self.volume);
            return None;
        };
        Some(Settings {
            start_time: Default::default(),
            start_position: Some(Duration::from_secs_f32(self.start_position)),
            region: self.region.into_region(),
            r#loop: Some(self.r#loop),
            volume: Some(volume),
            fade_in_tween: self.fade_in.into_interpolation(),
            panning: Some(self.panning),
            playback_rate: Some(kira::PlaybackRate(self.playback_rate as f64)),
            affected_by_time_dilation: Some(self.affected_by_time_dilation),
        })
    }
}

impl ToSettingsExt for EmitterSettings {
    type Settings = (SpatialTrackSettings, std::num::NonZero<u64>);
    type Defaults = Option<kira::track::SpatialTrackDistances>;
    fn into_settings_ext(self, defaults: Self::Defaults) -> Option<Self::Settings> {
        let mut state = ahash::AHasher::default();
        let d = self
            .distances
            .is_null()
            .not()
            .then(|| unsafe { self.distances.fields() }.cloned())
            .flatten();
        d.hash(&mut state);
        if !self.attenuation_function.is_null() {
            if self.attenuation_function.is_a::<LinearTween>() {
                let x = unsafe {
                    std::mem::transmute::<&Ref<Tween>, &Ref<LinearTween>>(
                        &self.attenuation_function,
                    )
                };
                let ty = unsafe { x.fields() }.unwrap();
                Some(ty).hash(&mut state);
            } else if self.attenuation_function.is_a::<ElasticTween>() {
                let x = unsafe {
                    std::mem::transmute::<&Ref<Tween>, &Ref<ElasticTween>>(
                        &self.attenuation_function,
                    )
                };
                let ty = unsafe { x.fields() }.unwrap();
                Some(ty).hash(&mut state);
            } else {
                fails!("invalid attenuation function");
                None::<Tween>.hash(&mut state);
            }
        } else {
            None::<Tween>.hash(&mut state);
        }
        let distances = self.distances.into_settings();
        let attenuation_function = self.attenuation_function.into_easing();
        self.enable_spatialization.hash(&mut state);
        self.persist_until_sounds_finish.hash(&mut state);
        let hash = state.finish();
        if hash == 0 {
            fails!("emitter settings hash should not be 0");
            return None;
        }
        Some((
            SpatialTrackSettings {
                distances: distances.unwrap_or(defaults.unwrap_or_default()),
                attenuation_function,
                spatialization_strength: if self.enable_spatialization {
                    0.75
                } else {
                    0.0
                },
                persist_until_sounds_finish: self.persist_until_sounds_finish,
                affected_by_reverb_mix: self.affected_by_reverb_mix,
                affected_by_environmental_preset: self.affected_by_environmental_preset,
            },
            // SAFETY: checked above
            unsafe { std::num::NonZeroU64::new_unchecked(hash) },
        ))
    }
}

impl ToSettings for EmitterDistances {
    type Settings = SpatialTrackDistances;
    fn into_settings(self) -> Option<Self::Settings> {
        Some(SpatialTrackDistances {
            min_distance: self.min_distance,
            max_distance: self.max_distance,
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

#[derive(Debug, Clone, Copy)]
pub struct TagName(CName);

impl TagName {
    pub fn try_new(value: CName) -> Result<Self, ValidationError> {
        match value.as_str() {
            "" | "None" => Err(ValidationError::InvalidTagName),
            _ => Ok(Self(value)),
        }
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Deref for TagName {
    type Target = CName;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TargetId(EntityId);

impl TargetId {
    pub fn try_new(value: EntityId) -> Result<Self, ValidationError> {
        if !value.is_defined()
            || get_player(GameInstance::new())
                .cast::<Entity>()
                .expect("PlayerPuppet inherits from Entity")
                .get_entity_id()
                == value
        {
            return Err(ValidationError::InvalidTargetId);
        }
        Ok(Self(value))
    }
}

impl Deref for TargetId {
    type Target = EntityId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for TargetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct TargetFootprint((SpatialTrackSettings, NonZero<u64>));

impl TargetFootprint {
    pub fn try_new(
        value: Ref<super::EmitterSettings>,
        entity_id: EntityId,
    ) -> Result<Option<Self>, Vec<ValidationError>> {
        use crate::engine::ToDistances;
        Ok(value.into_settings_ext(entity_id.to_distances()).map(Self))
    }
}

impl Deref for TargetFootprint {
    type Target = (SpatialTrackSettings, NonZero<u64>);

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct DynamicSoundEvent {
    base: IScriptable,
    pub(crate) id: OnceLock<ControlId>,
    pub(crate) name: Cell<EventName>,
    pub(crate) ext: RefCell<Option<Settings>>,
}

unsafe impl ScriptClass for DynamicSoundEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.DynamicSoundEvent";
}

impl AsRef<IScriptable> for DynamicSoundEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}

impl DynamicSoundEvent {
    pub(crate) fn create(name: CName, ext: Ref<AudioSettingsExt>) -> Ref<Self> {
        let Ok(name) = EventName::try_from(name) else {
            return Ref::default();
        };
        Ref::<Self>::new_with(|x| {
            x.name = Cell::new(name);
            x.ext.replace(Ref::clone(&ext).into_settings());
        })
        .unwrap_or_default()
    }
}
