//! [AudioSystemExt] surface API.

use audioware_manifest::ScnDialogLineType;
use kira::sound::PlaybackPosition;
use red4ext_rs::{
    class_kind::Native,
    log,
    types::{CName, EntityId, IScriptable, Opt, Ref},
    PluginOps, ScriptClass,
};

use crate::{
    engine::{AudioSettingsExt, Engine},
    types::Tween,
    Audioware,
};

/// Interop type for [Ext.reds](https://github.com/cyb3rpsych0s1s/audioware/blob/main/audioware/reds/Ext.reds).
#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct AudioSystemExt {
    base: IScriptable,
}

unsafe impl ScriptClass for AudioSystemExt {
    type Kind = Native;
    const NAME: &'static str = "AudioSystemExt";
}

impl AudioSystemExt {
    pub fn play(
        &self,
        sound_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        line_type: Opt<ScnDialogLineType>,
        ext: Ref<AudioSettingsExt>,
    ) {
        Engine::play_with(sound_name, entity_id, emitter_name, line_type, ext);
    }
    pub fn stop(
        &self,
        event_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        tween: Ref<Tween>,
    ) {
        Engine::stop(event_name, entity_id, emitter_name, tween);
    }
    pub fn switch(
        &self,
        switch_name: CName,
        switch_value: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        switch_name_tween: Ref<Tween>,
        switch_value_settings: Ref<AudioSettingsExt>,
    ) {
        Engine::switch(
            switch_name,
            switch_value,
            entity_id,
            emitter_name,
            switch_name_tween,
            switch_value_settings,
        );
    }
    pub fn play_over_the_phone(&self, event_name: CName, emitter_name: CName, gender: CName) {
        Engine::play_over_the_phone(event_name, emitter_name, gender);
    }
    pub fn is_registered_emitter(&self, entity_id: EntityId) -> bool {
        Engine::is_registered_emitter(entity_id)
    }
    pub fn emitters_count(&self) -> i32 {
        Engine::emitters_count()
    }
    pub fn play_on_emitter(
        &self,
        sound_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: Ref<Tween>,
    ) {
        Engine::play_on_emitter(sound_name, entity_id, emitter_name, tween);
    }
    pub fn stop_on_emitter(
        &self,
        sound_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: Ref<Tween>,
    ) {
        Engine::stop_on_emitter(sound_name, entity_id, emitter_name, tween);
    }
    pub fn on_emitter_dies(&self, entity_id: EntityId) {
        Engine::on_emitter_dies(entity_id);
    }
}

/// Any type which can be merged with [settings](AudioSettingsExt).
pub trait MergeArgs {
    fn merge_args(self, ext: &Ref<AudioSettingsExt>) -> Self;
}

macro_rules! impl_merge_args {
    ($into:path) => {
        impl MergeArgs for $into {
            /// Merge defined [AudioSettingsExt] values when valid.
            fn merge_args(mut self, ext: &Ref<AudioSettingsExt>) -> Self {
                if ext.is_null() {
                    return self;
                }
                let fields = unsafe { ext.fields() }.unwrap();
                if let Some(start_position) = fields.start_position {
                    match start_position {
                        PlaybackPosition::Seconds(x) => {
                            if x < self.duration().as_secs_f64() {
                                self = self.start_position(PlaybackPosition::Seconds(x));
                            }
                        }
                        PlaybackPosition::Samples(_) => {
                            log::warn!(
                                Audioware::env(),
                                "Setting start position in samples is not supported."
                            );
                        }
                    };
                }
                if let Some(region) = fields.region {
                    match (region.start, region.end) {
                        (
                            PlaybackPosition::Seconds(start),
                            kira::sound::EndPosition::EndOfAudio,
                        ) => {
                            if start > 0. && start < self.duration().as_secs_f64() {
                                let value = kira::sound::Region {
                                    start: PlaybackPosition::Seconds(start),
                                    end: kira::sound::EndPosition::EndOfAudio,
                                };
                                if fields.r#loop.unwrap_or_default() {
                                    self = self.loop_region(value);
                                } else {
                                    self = self.slice(value);
                                }
                            }
                        }
                        (
                            PlaybackPosition::Seconds(start),
                            kira::sound::EndPosition::Custom(PlaybackPosition::Seconds(end)),
                        ) => {
                            if start >= 0.0
                                && start < self.duration().as_secs_f64()
                                && end > 0.0
                                && end <= self.duration().as_secs_f64()
                                && start < end
                            {
                                let value = kira::sound::Region {
                                    start: PlaybackPosition::Seconds(start),
                                    end: kira::sound::EndPosition::Custom(
                                        PlaybackPosition::Seconds(end),
                                    ),
                                };
                                if fields.r#loop.unwrap_or_default() {
                                    self = self.loop_region(value);
                                } else {
                                    self = self.slice(value);
                                }
                            }
                        }
                        _ => {
                            log::warn!(
                                Audioware::env(),
                                "Setting region in samples unit is not supported."
                            );
                        }
                    }
                }
                if let Some(volume) = fields.volume {
                    if volume.as_decibels() <= 85.0 {
                        self = self.volume(volume);
                    } else {
                        log::warn!(Audioware::env(), "Volume must not be higher than 85 dB.");
                    }
                }
                if let Some(fade_in_tween) = fields.fade_in_tween {
                    self = self.fade_in_tween(fade_in_tween);
                }
                if let Some(panning) = fields.panning {
                    if (0.0..=1.0).contains(&panning) {
                        self = self.panning(panning);
                    } else {
                        log::warn!(
                            Audioware::env(),
                            "Panning must be between 0.0 and 1.0 (inclusive)."
                        );
                    }
                }
                if let Some(playback_rate) = fields.playback_rate {
                    self = self.playback_rate(playback_rate);
                }
                self
            }
        }
    };
}

impl_merge_args!(kira::sound::static_sound::StaticSoundData);
impl_merge_args!(kira::sound::streaming::StreamingSoundData<kira::sound::FromFileError>);
