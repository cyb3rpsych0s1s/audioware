use std::cell::Cell;

use kira::{sound::PlaybackPosition, Volume};
use red4ext_rs::{
    class_kind::Native,
    log,
    types::{IScriptable, Ref},
    PluginOps, ScriptClass,
};

use crate::Audioware;

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct ArgsBuilder {
    base: IScriptable,
    start_position: Cell<Option<f32>>,
    volume: Cell<Option<f32>>,
}

unsafe impl ScriptClass for ArgsBuilder {
    type Kind = Native;
    const NAME: &'static str = "Audioware.ArgsBuilder";
}

impl ArgsBuilder {
    pub fn create() -> Ref<Self> {
        Ref::<Self>::new().unwrap_or_default()
    }
    pub fn set_start_position(&self, value: f32) -> Ref<Self> {
        self.start_position.set(Some(value));
        log::info!(Audioware::env(), "set start_position to {value}");
        Ref::<ArgsBuilder>::new_with(|x| {
            x.start_position = self.start_position.clone();
            x.volume = self.volume.clone();
        })
        .unwrap_or_default()
    }
    pub fn set_volume(&self, value: f32) -> Ref<Self> {
        self.volume.set(Some(value));
        log::info!(Audioware::env(), "set volume to {value}");
        Ref::<ArgsBuilder>::new_with(|x| {
            x.start_position = self.start_position.clone();
            x.volume = self.volume.clone();
        })
        .unwrap_or_default()
    }
    pub fn build(&self) -> Ref<ArgsExt> {
        Ref::<ArgsExt>::new_with(|x| {
            if let Some(start_position) = self.start_position.get() {
                x.start_position = Some(PlaybackPosition::Seconds(start_position.into()));
            }
            if let Some(volume) = self.volume.get() {
                x.volume = Some(Volume::Amplitude(volume.into()));
            }
        })
        .unwrap_or_default()
    }
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct ArgsExt {
    base: IScriptable,
    pub start_position: Option<PlaybackPosition>,
    pub volume: Option<Volume>,
}

unsafe impl ScriptClass for ArgsExt {
    type Kind = Native;
    const NAME: &'static str = "Audioware.ArgsExt";
}
