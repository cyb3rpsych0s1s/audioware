use std::{cell::Cell, time::Duration};

use kira::{
    sound::{EndPosition, PlaybackPosition, PlaybackRate},
    Volume,
};
use red4ext_rs::{
    class_kind::Native,
    log,
    types::{IScriptable, Ref},
    NativeRepr, PluginOps, ScriptClass,
};

use crate::{
    types::{Easing, ElasticTween, LinearTween, Tween},
    Audioware,
};

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct AudioRegion {
    starts: Cell<Option<f32>>,
    ends: Cell<Option<f32>>,
}

impl AudioRegion {
    pub fn set_start(&self, value: f32) {
        self.starts.set(Some(value));
    }
    pub fn set_end(&self, value: f32) {
        self.ends.set(Some(value));
    }
}

unsafe impl NativeRepr for AudioRegion {
    const NAME: &'static str = "Audioware.AudioRegion";
}

unsafe impl ScriptClass for AudioRegion {
    type Kind = Native;
    const NAME: &'static str = <Self as NativeRepr>::NAME;
}

impl From<AudioRegion> for kira::sound::Region {
    fn from(value: AudioRegion) -> Self {
        Self {
            start: if let Some(starts) = value.starts.get() {
                PlaybackPosition::Seconds(starts as f64)
            } else {
                PlaybackPosition::Seconds(0.0)
            },
            end: if let Some(ends) = value.ends.get() {
                EndPosition::Custom(PlaybackPosition::Seconds(ends as f64))
            } else {
                EndPosition::EndOfAudio
            },
        }
    }
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct AudioSettingsExtBuilder {
    base: IScriptable,
    start_position: Cell<Option<f32>>,
    loop_region_starts: Cell<Option<f32>>,
    loop_region_ends: Cell<Option<f32>>,
    volume: Cell<Option<f32>>,
    fade_in_tween_start_time: Cell<Option<f32>>,
    fade_in_tween_duration: Cell<Option<f32>>,
    fade_in_tween_linear: Cell<Option<bool>>,
    fade_in_tween_elastic_easing: Cell<Option<Easing>>,
    fade_in_tween_elastic_value: Cell<Option<f32>>,
    panning: Cell<Option<f32>>,
    playback_rate: Cell<Option<f32>>,
}

unsafe impl ScriptClass for AudioSettingsExtBuilder {
    type Kind = Native;
    const NAME: &'static str = "Audioware.AudioSettingsExtBuilder";
}

impl AudioSettingsExtBuilder {
    pub fn create() -> Ref<Self> {
        Ref::<Self>::new().unwrap_or_default()
    }
    pub fn set_start_position(&self, value: f32) {
        self.start_position.set(Some(value));
        log::info!(Audioware::env(), "set start_position to {value}");
    }
    pub fn set_loop_region_starts(&self, value: f32) {
        self.loop_region_starts.set(Some(value));
        log::info!(Audioware::env(), "set loop_region starts to {value}");
    }
    pub fn set_loop_region_ends(&self, value: f32) {
        self.loop_region_ends.set(Some(value));
        log::info!(Audioware::env(), "set loop_region ends to {value}");
    }
    pub fn set_volume(&self, value: f32) {
        self.volume.set(Some(value));
        log::info!(Audioware::env(), "set volume to {value}");
    }
    pub fn set_fade_in_tween(&self, value: Ref<Tween>) {
        if value.is_null() {
            self.fade_in_tween_start_time.set(None);
            self.fade_in_tween_duration.set(None);
            self.fade_in_tween_linear.set(None);
            self.fade_in_tween_elastic_easing.set(None);
            self.fade_in_tween_elastic_value.set(None);
        } else if value.is_a::<LinearTween>() {
            let linear = value.clone().cast::<LinearTween>().unwrap();
            let linear = unsafe { linear.fields().unwrap() };
            self.fade_in_tween_start_time.set(Some(linear.start_time()));
            self.fade_in_tween_duration.set(Some(linear.duration()));
            self.fade_in_tween_linear.set(Some(true));
            self.fade_in_tween_elastic_easing.set(None);
            self.fade_in_tween_elastic_value.set(None);
        } else if value.is_a::<ElasticTween>() {
            let elastic = value.clone().cast::<ElasticTween>().unwrap();
            let elastic = unsafe { elastic.fields().unwrap() };
            self.fade_in_tween_start_time
                .set(Some(elastic.start_time()));
            self.fade_in_tween_duration.set(Some(elastic.duration()));
            self.fade_in_tween_linear.set(Some(false));
            self.fade_in_tween_elastic_easing.set(Some(elastic.easing));
            self.fade_in_tween_elastic_value.set(Some(elastic.value));
        } else {
            log::error!(Audioware::env(), "unknown tween variant");
        }
        log::info!(Audioware::env(), "set fade_in_tween");
    }
    pub fn set_panning(&self, value: f32) {
        self.panning.set(Some(value));
        log::info!(Audioware::env(), "set panning to {value}");
    }
    pub fn set_playback_rate(&self, value: f32) {
        self.playback_rate.set(Some(value));
        log::info!(Audioware::env(), "set playback_rate to {value}");
    }
    pub fn build(&self) -> Ref<AudioSettingsExt> {
        Ref::<AudioSettingsExt>::new_with(|x| {
            log::info!(Audioware::env(), "build...");
            if let Some(start_position) = self.start_position.get() {
                x.start_position = Some(PlaybackPosition::Seconds(start_position.into()));
            }
            match (self.loop_region_starts.get(), self.loop_region_ends.get()) {
                (Some(start), Some(end)) => {
                    x.loop_region = Some(kira::sound::Region {
                        start: PlaybackPosition::Seconds(start.into()),
                        end: EndPosition::Custom(PlaybackPosition::Seconds(end.into())),
                    });
                }
                (Some(start), None) => {
                    x.loop_region = Some(kira::sound::Region {
                        start: PlaybackPosition::Seconds(start.into()),
                        end: EndPosition::EndOfAudio,
                    });
                }
                (None, Some(end)) => {
                    x.loop_region = Some(kira::sound::Region {
                        start: PlaybackPosition::Seconds(0.0),
                        end: EndPosition::Custom(PlaybackPosition::Seconds(end.into())),
                    });
                }
                _ => {}
            };
            if let Some(volume) = self.volume.get() {
                x.volume = Some(Volume::Amplitude(volume.into()));
            }
            if self.fade_in_tween_linear.get().is_some() {
                let start_time = self.fade_in_tween_start_time.get();
                let duration = self.fade_in_tween_duration.get();
                if self.fade_in_tween_linear.get().unwrap() {
                    x.fade_in_tween = Some(kira::tween::Tween {
                        start_time: start_time
                            .map(Duration::from_secs_f32)
                            .map(kira::StartTime::Delayed)
                            .unwrap_or_default(),
                        duration: duration.map(Duration::from_secs_f32).unwrap_or_default(),
                        easing: kira::tween::Easing::Linear,
                    });
                } else {
                    let curve = self.fade_in_tween_elastic_easing.get().unwrap();
                    let value = self.fade_in_tween_elastic_value.get().unwrap() as f64;
                    x.fade_in_tween = Some(kira::tween::Tween {
                        start_time: start_time
                            .map(Duration::from_secs_f32)
                            .map(kira::StartTime::Delayed)
                            .unwrap_or_default(),
                        duration: duration.map(Duration::from_secs_f32).unwrap_or_default(),
                        easing: match curve {
                            Easing::InPowf => kira::tween::Easing::InPowf(value),
                            Easing::OutPowf => kira::tween::Easing::OutPowf(value),
                            Easing::InOutPowf => kira::tween::Easing::InOutPowf(value),
                        },
                    });
                }
            }
            if let Some(panning) = self.panning.get() {
                x.panning = Some(panning.into());
            }
            if let Some(playback_rate) = self.playback_rate.get() {
                x.playback_rate = Some(PlaybackRate::Factor(playback_rate.into()));
            }
            log::info!(Audioware::env(), "built!");
        })
        .unwrap_or_default()
    }
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct AudioSettingsExt {
    base: IScriptable,
    pub start_position: Option<PlaybackPosition>,
    pub loop_region: Option<kira::sound::Region>,
    pub volume: Option<Volume>,
    pub fade_in_tween: Option<kira::tween::Tween>,
    pub panning: Option<f64>,
    pub playback_rate: Option<PlaybackRate>,
}

unsafe impl ScriptClass for AudioSettingsExt {
    type Kind = Native;
    const NAME: &'static str = "Audioware.AudioSettingsExt";
}
