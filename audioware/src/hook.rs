use std::borrow::BorrowMut;

use audioware_mem::native_func;
use red4ext_rs::types::{CName, EntityId};

use crate::addresses::{ON_AUDIOSYSTEM_PLAY, ON_AUDIOSYSTEM_STOP, ON_AUDIOSYSTEM_SWITCH};

pub fn is_audioware(event_name: CName, _entity_id: EntityId, _emitter_name: CName) -> bool {
    if let Ok(exists) = crate::engine::banks::exists(event_name.clone()) {
        return exists;
    } else {
        red4ext_rs::error!("unable to find sound {event_name} existence in banks");
    }
    false
}

pub fn should_switch(
    _switch_name: CName,
    _switch_value: CName,
    _entity_id: EntityId,
    _emitter_name: CName,
) -> bool {
    false
}

pub fn custom_engine_play(event_name: CName, entity_id: EntityId, emitter_name: CName) {
    // red4ext_rs::info!(
    //     "call custom engine Play method with: event_name {}, entity_id {:#?}, emitter_name {}",
    //     red4ext_rs::ffi::resolve_cname(&event_name),
    //     entity_id,
    //     red4ext_rs::ffi::resolve_cname(&emitter_name)
    // );
    let entity_id = if entity_id == EntityId::default() {
        None
    } else {
        Some(entity_id)
    };
    let emitter_name = if emitter_name == CName::default() {
        None
    } else {
        Some(emitter_name)
    };
    crate::engine::play(event_name, entity_id, emitter_name);
}

pub fn custom_engine_stop(event_name: CName, entity_id: EntityId, emitter_name: CName) {
    // red4ext_rs::info!(
    //     "call custom engine Stop method with: event_name {}, entity_id {:#?}, emitter_name {}",
    //     red4ext_rs::ffi::resolve_cname(&event_name),
    //     entity_id,
    //     red4ext_rs::ffi::resolve_cname(&emitter_name)
    // );
    let entity_id = if entity_id == EntityId::default() {
        None
    } else {
        Some(entity_id)
    };
    let emitter_name = if emitter_name == CName::default() {
        None
    } else {
        Some(emitter_name)
    };
    crate::engine::stop(event_name, entity_id, emitter_name);
}

pub fn custom_engine_switch(
    _switch_name: CName,
    _switch_value: CName,
    _entity_id: EntityId,
    _emitter_name: CName,
) {
}

native_func!(
    HookAudioSystemPlay,
    ON_AUDIOSYSTEM_PLAY,
    HOOK_ON_AUDIOSYSTEM_PLAY,
    on_audiosystem_play,
    (sound_name: CName, entity_id: EntityId, emitter_name: CName) -> (),
    is_audioware,
    custom_engine_play
);

native_func!(
    HookAudioSystemStop,
    ON_AUDIOSYSTEM_STOP,
    HOOK_ON_AUDIOSYSTEM_STOP,
    on_audiosystem_stop,
    (sound_name: CName, entity_id: EntityId, emitter_name: CName) -> (),
    is_audioware,
    custom_engine_stop
);

native_func!(
    HookAudioSystemSwitch,
    ON_AUDIOSYSTEM_SWITCH,
    HOOK_ON_AUDIOSYSTEM_SWITCH,
    on_audiosystem_switch,
    (switch_name: CName, switch_value: CName, entity_id: EntityId, emitter_name: CName) -> (),
    should_switch,
    custom_engine_switch
);
