use audioware_manifest::ScnDialogLineType;
use red4ext_rs::{
    class_kind::Native,
    types::{CName, EntityId, Opt, Ref},
    NativeRepr, ScriptClass,
};

use crate::{engine::Engine, types::Tween};

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct AudioSystemExt;

unsafe impl NativeRepr for AudioSystemExt {
    const NAME: &'static str = "AudioSystemExt";
}

unsafe impl ScriptClass for AudioSystemExt {
    type Kind = Native;
    const NAME: &'static str = "AudioSystemExt";
}

impl AudioSystemExt {
    pub fn play(
        self,
        sound_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        line_type: Opt<ScnDialogLineType>,
        tween: Ref<Tween>,
    ) {
        Engine::play(sound_name, entity_id, emitter_name, line_type, tween);
    }
    pub fn stop(
        self,
        event_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        tween: Ref<Tween>,
    ) {
        Engine::stop(event_name, entity_id, emitter_name, tween);
    }
    pub fn switch(
        self,
        switch_name: CName,
        switch_value: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        switch_name_tween: Ref<Tween>,
        switch_value_tween: Ref<Tween>,
    ) {
        Engine::switch(
            switch_name,
            switch_value,
            entity_id,
            emitter_name,
            switch_name_tween,
            switch_value_tween,
        );
    }
    pub fn play_over_the_phone(self, event_name: CName, emitter_name: CName, gender: CName) {
        Engine::play_over_the_phone(event_name, emitter_name, gender);
    }
    #[allow(clippy::wrong_self_convention)]
    pub fn is_registered_emitter(self, entity_id: EntityId) -> bool {
        Engine::is_registered_emitter(entity_id)
    }
    pub fn emitters_count(self) -> i32 {
        Engine::emitters_count()
    }
    pub fn play_on_emitter(
        self,
        sound_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: Ref<Tween>,
    ) {
        Engine::play_on_emitter(sound_name, entity_id, emitter_name, tween);
    }
    pub fn stop_on_emitter(
        self,
        sound_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: Ref<Tween>,
    ) {
        Engine::stop_on_emitter(sound_name, entity_id, emitter_name, tween);
    }
}
