//! [AudioSystemExt] surface API.

use audioware_manifest::{LocaleExt, PlayerGender, ScnDialogLineType};
use red4ext_rs::{
    class_kind::Native,
    log,
    types::{CName, EntityId, IScriptable, Opt, Ref, StaticArray},
    PluginOps, ScriptClass,
};

use crate::{
    engine::{AudioSettingsExt, Command, Engine},
    types::Tween,
    Audioware, AUDIOWARE_VERSION,
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
        Engine::send(Command::PlayExt {
            sound_name,
            entity_id,
            emitter_name,
            line_type,
            ext: ext.into(),
        });
    }
    pub fn stop(
        &self,
        event_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        tween: Ref<Tween>,
    ) {
        Engine::send(Command::Stop {
            event_name,
            entity_id,
            emitter_name,
            tween: tween.into(),
        });
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
        Engine::send(Command::Switch {
            switch_name,
            switch_value,
            entity_id,
            emitter_name,
            switch_name_tween: switch_name_tween.into(),
            switch_value_settings: switch_value_settings.into(),
        })
    }
    pub fn play_over_the_phone(&self, event_name: CName, emitter_name: CName, gender: CName) {
        if let Ok(gender) = gender.try_into() {
            Engine::send(Command::PlayOverThePhone {
                event_name,
                emitter_name,
                gender,
            });
        } else {
            log::warn!(Audioware::env(), "invalid gender: {gender}");
        }
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
        Engine::send(Command::PlayOnEmitter {
            sound_name,
            entity_id,
            emitter_name,
            tween: tween.into(),
        });
    }
    pub fn stop_on_emitter(
        &self,
        sound_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: Ref<Tween>,
    ) {
        Engine::send(Command::StopOnEmitter {
            event_name: sound_name,
            entity_id,
            emitter_name,
            tween: tween.into(),
        })
    }
    pub fn on_emitter_dies(&self, entity_id: EntityId) {
        Engine::on_emitter_dies(entity_id);
    }
    pub fn semantic_version(&self) -> StaticArray<u16, 5> {
        StaticArray::from(AUDIOWARE_VERSION)
    }
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
        // Banks::duration(
        //     &event_name,
        //     locale
        //         .into_option()
        //         .and_then(|x| x.try_into().ok())
        //         .unwrap_or_default(),
        //     gender.unwrap_or_default(),
        //     total.unwrap_or_default(),
        // )
        todo!()
    }
}
