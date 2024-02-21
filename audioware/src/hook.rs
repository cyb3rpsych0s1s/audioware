use audioware_macros::NativeFunc;
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

pub fn should_switch(params: &(CName, CName, EntityId, CName)) -> bool {
    let (switch_name, switch_value, ..) = params.clone();
    match crate::engine::banks::exist(&[switch_name.clone(), switch_value.clone()]) {
        Ok(exist) => exist,
        Err(_) => {
            red4ext_rs::error!(
                "unable to find sounds existence in banks ({}, {})",
                switch_name,
                switch_value
            );
            false
        }
    }
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

pub fn custom_engine_switch(params: (CName, CName, EntityId, CName)) {
    let (switch_name, switch_value, entity_id, emitter_name) = params;
    custom_engine_stop((switch_name, entity_id.clone(), emitter_name.clone()));
    custom_engine_play((switch_value, entity_id, emitter_name));
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

#[derive(NativeFunc)]
#[hook(
    offset = ON_AUDIOSYSTEM_SWITCH,
    inputs = "(CName, CName, EntityId, CName)",
    allow = "should_switch",
    detour = "custom_engine_switch"
)]
pub struct HookAudioSystemSwitch;
