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
    NativeRepr, ScriptClass,
    class_kind::{Native, Scripted},
    types::{CName, EntityId, GameInstance, IScriptable, Opt, Ref, StaticArray},
};

use crate::{
    AUDIOWARE_VERSION, AsEntity, ControlId, ElasticTween, EmitterDistances, EmitterSettings,
    Entity, Event, EventName, LinearTween, ToEasing, Tween, abi::fails, engine::Engine,
    error::ValidationError, get_player, utils::warns,
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
                enable_occlusion: self.enable_occlusion,
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

#[derive(Debug, Clone, Default, Copy)]
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

impl std::fmt::Display for TagName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "tag:{}", self.0)
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
    base: Event,
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
        self.base.as_ref()
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

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct DynamicEmitterEvent {
    base: Event,
    pub(crate) id: OnceLock<ControlId>,
    pub(crate) name: Cell<EventName>,
    pub(crate) tag_name: Cell<TagName>,
    pub(crate) ext: RefCell<Option<Settings>>,
}

unsafe impl ScriptClass for DynamicEmitterEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.DynamicEmitterEvent";
}

impl DynamicEmitterEvent {
    pub(crate) fn create(name: CName, tag_name: CName, ext: Ref<AudioSettingsExt>) -> Ref<Self> {
        let Ok(name) = EventName::try_from(name) else {
            return Ref::default();
        };
        let Ok(tag_name) = TagName::try_new(tag_name) else {
            return Ref::default();
        };
        Ref::<Self>::new_with(|x| {
            x.name = Cell::new(name);
            x.tag_name = Cell::new(tag_name);
            x.ext.replace(Ref::clone(&ext).into_settings());
        })
        .unwrap_or_default()
    }
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct DynamicEffect {
    base: IScriptable,
    pub(crate) id: OnceLock<ControlId>,
}

unsafe impl ScriptClass for DynamicEffect {
    type Kind = Native;
    const NAME: &'static str = "Audioware.DynamicEffect";
}

impl AsRef<IScriptable> for DynamicEffect {
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct DynamicEQ {
    pub(crate) base: DynamicEffect,
    pub(crate) kind: Cell<EqFilterKind>,
    pub(crate) frequency: Cell<f32>,
    pub(crate) gain: Cell<f32>,
    pub(crate) q: Cell<f32>,
}

unsafe impl ScriptClass for DynamicEQ {
    type Kind = Native;
    const NAME: &'static str = "Audioware.DynamicEQ";
}

impl DynamicEQ {
    pub(crate) fn create(kind: EqFilterKind, frequency: f32, gain: f32, q: f32) -> Ref<Self> {
        if q <= 0.0 {
            warns!("frequency range width (a.k.a 'Q'): cannot be lower or equals to 0.");
            return Default::default();
        }
        Ref::<Self>::new_with(|x| {
            x.kind = Cell::new(kind);
            x.frequency = Cell::new(frequency);
            x.gain = Cell::new(gain);
            x.q = Cell::new(q);
        })
        .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum EqFilterKind {
    #[default]
    Bell = 0,
    LowShelf = 1,
    HighShelf = 2,
}

unsafe impl NativeRepr for EqFilterKind {
    const NAME: &'static str = "Audioware.EqFilterKind";
}

impl From<self::EqFilterKind> for kira::effect::eq_filter::EqFilterKind {
    fn from(value: self::EqFilterKind) -> Self {
        match value {
            self::EqFilterKind::Bell => kira::effect::eq_filter::EqFilterKind::Bell,
            self::EqFilterKind::LowShelf => kira::effect::eq_filter::EqFilterKind::LowShelf,
            self::EqFilterKind::HighShelf => kira::effect::eq_filter::EqFilterKind::HighShelf,
        }
    }
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct DynamicDistortion {
    pub(crate) base: DynamicEffect,
    pub(crate) kind: Cell<DistortionKind>,
    pub(crate) drive: Cell<f32>,
    pub(crate) mix: Cell<f32>,
}

unsafe impl ScriptClass for DynamicDistortion {
    type Kind = Native;
    const NAME: &'static str = "Audioware.DynamicDistortion";
}

impl DynamicDistortion {
    pub(crate) fn create(kind: DistortionKind, drive: f32, mix: f32) -> Ref<Self> {
        if !(0.0..=1.0).contains(&mix) {
            warns!("invalid mix ({mix}): must be between 0.0 and 1.0.");
            return Default::default();
        }
        Ref::<Self>::new_with(|x| {
            x.kind = Cell::new(kind);
            x.drive = Cell::new(drive);
            x.mix = Cell::new(mix);
        })
        .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum DistortionKind {
    #[default]
    HardClip = 0,
    SoftClip = 1,
}

impl From<self::DistortionKind> for kira::effect::distortion::DistortionKind {
    fn from(value: self::DistortionKind) -> Self {
        match value {
            self::DistortionKind::HardClip => kira::effect::distortion::DistortionKind::HardClip,
            self::DistortionKind::SoftClip => kira::effect::distortion::DistortionKind::SoftClip,
        }
    }
}

unsafe impl NativeRepr for DistortionKind {
    const NAME: &'static str = "Audioware.DistortionKind";
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct DynamicDelay {
    pub(crate) base: DynamicEffect,
    pub(crate) feedback: Cell<f32>,
    pub(crate) delay_time: Cell<Duration>,
    pub(crate) mix: Cell<f32>,
}

unsafe impl ScriptClass for DynamicDelay {
    type Kind = Native;
    const NAME: &'static str = "Audioware.DynamicDelay";
}

impl DynamicDelay {
    pub(crate) fn create(feedback: f32, delay_time: f32, mix: f32) -> Ref<Self> {
        if !(0.0..=1.0).contains(&mix) {
            warns!("invalid mix ({mix}): must be between 0.0 and 1.0.");
            return Default::default();
        }
        let Ok(delay_time) = Duration::try_from_secs_f32(delay_time).inspect_err(|e| {
            warns!("invalid delay time ({delay_time}): {e}");
        }) else {
            return Default::default();
        };
        Ref::<Self>::new_with(|x| {
            x.feedback = Cell::new(feedback);
            x.delay_time = Cell::new(delay_time);
            x.mix = Cell::new(mix);
        })
        .unwrap_or_default()
    }
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct DynamicCompressor {
    pub(crate) base: DynamicEffect,
    pub(crate) threshold: Cell<f32>,
    pub(crate) ratio: Cell<f32>,
    pub(crate) attack_duration: Cell<Duration>,
    pub(crate) release_duration: Cell<Duration>,
    pub(crate) makeup_gain: Cell<f32>,
    pub(crate) mix: Cell<f32>,
}

unsafe impl ScriptClass for DynamicCompressor {
    type Kind = Native;
    const NAME: &'static str = "Audioware.DynamicCompressor";
}

impl DynamicCompressor {
    pub(crate) fn create(
        threshold: f32,
        ratio: f32,
        attack_duration: f32,
        release_duration: f32,
        makeup_gain: f32,
        mix: f32,
    ) -> Ref<Self> {
        if ratio < 0. {
            warns!("invalid ratio ({ratio}): cannot be lower than 0.");
            return Default::default();
        }
        let Ok(attack_duration) = Duration::try_from_secs_f32(attack_duration).inspect_err(|e| {
            warns!("invalid attack duration ({attack_duration}): {e}");
        }) else {
            return Default::default();
        };
        let Ok(release_duration) = Duration::try_from_secs_f32(release_duration).inspect_err(|e| {
            warns!("invalid release duration ({release_duration}): {e}");
        }) else {
            return Default::default();
        };
        if !(0.0..=1.0).contains(&mix) {
            warns!("invalid mix ({mix}): must be between 0.0 and 1.0.");
            return Default::default();
        }
        Ref::<Self>::new_with(|x| {
            x.threshold = Cell::new(threshold);
            x.ratio = Cell::new(ratio);
            x.attack_duration = Cell::new(attack_duration);
            x.release_duration = Cell::new(release_duration);
            x.makeup_gain = Cell::new(makeup_gain);
            x.mix = Cell::new(mix);
        })
        .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum FilterMode {
    #[default]
    LowPass = 0,
    BandPass = 1,
    HighPass = 2,
    Notch = 3,
}

unsafe impl NativeRepr for FilterMode {
    const NAME: &'static str = "Audioware.FilterMode";
}

impl From<self::FilterMode> for kira::effect::filter::FilterMode {
    fn from(value: self::FilterMode) -> Self {
        match value {
            self::FilterMode::LowPass => kira::effect::filter::FilterMode::LowPass,
            self::FilterMode::BandPass => kira::effect::filter::FilterMode::BandPass,
            self::FilterMode::HighPass => kira::effect::filter::FilterMode::HighPass,
            self::FilterMode::Notch => kira::effect::filter::FilterMode::Notch,
        }
    }
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct DynamicFilter {
    pub(crate) base: DynamicEffect,
    pub(crate) mode: Cell<FilterMode>,
    pub(crate) cutoff: Cell<f32>,
    pub(crate) resonance: Cell<f32>,
    pub(crate) mix: Cell<f32>,
}

unsafe impl ScriptClass for DynamicFilter {
    type Kind = Native;
    const NAME: &'static str = "Audioware.DynamicFilter";
}

impl DynamicFilter {
    pub(crate) fn create(mode: FilterMode, cutoff: f32, resonance: f32, mix: f32) -> Ref<Self> {
        if !(0.0..=1.0).contains(&mix) {
            warns!("invalid mix ({mix}): must be between 0.0 and 1.0.");
            return Default::default();
        }
        Ref::<Self>::new_with(|x| {
            x.mode = Cell::new(mode);
            x.cutoff = Cell::new(cutoff);
            x.resonance = Cell::new(resonance);
            x.mix = Cell::new(mix);
        })
        .unwrap_or_default()
    }
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct DynamicReverb {
    pub(crate) base: DynamicEffect,
    pub(crate) feedback: Cell<f32>,
    pub(crate) damping: Cell<f32>,
    pub(crate) stereo_width: Cell<f32>,
    pub(crate) mix: Cell<f32>,
}

unsafe impl ScriptClass for DynamicReverb {
    type Kind = Native;
    const NAME: &'static str = "Audioware.DynamicReverb";
}

impl DynamicReverb {
    pub(crate) fn create(feedback: f32, damping: f32, stereo_width: f32, mix: f32) -> Ref<Self> {
        if !(0.0..=1.0).contains(&feedback) {
            warns!("invalid feedback ({feedback}): must be between 0.0 and 1.0.");
            return Default::default();
        }
        if !(0.0..=1.0).contains(&stereo_width) {
            warns!("invalid stereo width ({stereo_width}): must be between 0.0 and 1.0.");
            return Default::default();
        }
        if !(0.0..=1.0).contains(&mix) {
            warns!("invalid mix ({mix}): must be between 0.0 and 1.0.");
            return Default::default();
        }
        Ref::<Self>::new_with(|x| {
            x.feedback = Cell::new(feedback);
            x.damping = Cell::new(damping);
            x.stereo_width = Cell::new(stereo_width);
            x.mix = Cell::new(mix);
        })
        .unwrap_or_default()
    }
}
