use std::borrow::BorrowMut;

use audioware_macros::NativeFunc;
use audioware_mem::native_func;
use red4ext_rs::types::{CName, EntityId};

use crate::addresses::{ON_AUDIOSYSTEM_PLAY, ON_AUDIOSYSTEM_STOP, ON_AUDIOSYSTEM_SWITCH};

pub fn is_audioware(params: &(CName, EntityId, CName)) -> bool {
    let (event_name, ..) = params;
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

pub fn custom_engine_play(params: (CName, EntityId, CName)) {
    // red4ext_rs::info!(
    //     "call custom engine Play method with: event_name {}, entity_id {:#?}, emitter_name {}",
    //     red4ext_rs::ffi::resolve_cname(&event_name),
    //     entity_id,
    //     red4ext_rs::ffi::resolve_cname(&emitter_name)
    // );
    let (event_name, entity_id, emitter_name) = params;
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

pub fn custom_engine_stop(params: (CName, EntityId, CName)) {
    // red4ext_rs::info!(
    //     "call custom engine Stop method with: event_name {}, entity_id {:#?}, emitter_name {}",
    //     red4ext_rs::ffi::resolve_cname(&event_name),
    //     entity_id,
    //     red4ext_rs::ffi::resolve_cname(&emitter_name)
    // );
    let (event_name, entity_id, emitter_name) = params;
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

#[derive(NativeFunc)]
#[hook(
    offset = ON_AUDIOSYSTEM_PLAY,
    inputs = "(CName, EntityId, CName)",
    allow = "is_audioware",
    detour = "custom_engine_play"
)]
pub struct HookAudioSystemPlay;

#[derive(NativeFunc)]
#[hook(
    offset = ON_AUDIOSYSTEM_STOP,
    inputs = "(CName, EntityId, CName)",
    allow = "is_audioware",
    detour = "custom_engine_stop"
)]
pub struct HookAudioSystemStop;

native_func!(
    HookAudioSystemSwitch,
    ON_AUDIOSYSTEM_SWITCH,
    HOOK_ON_AUDIOSYSTEM_SWITCH,
    on_audiosystem_switch,
    (switch_name: CName, switch_value: CName, entity_id: EntityId, emitter_name: CName) -> (),
    should_switch,
    custom_engine_switch
);
