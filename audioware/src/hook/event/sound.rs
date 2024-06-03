use std::pin::Pin;

use crate::hook::address::{
    ON_SOUNDPARAMETER_EVENT, ON_SOUNDSWITCH_EVENT, ON_STOPSOUND_EVENT, ON_STOPTAGGEDSOUNDS_EVENT,
};

use super::super::address::ON_PLAYSOUND_EVENT;
use audioware_macros::NativeHandler;
use audioware_mem::FromMemory;
use audioware_sys::interop::{
    audio::{PlaySound, SoundParameter, StopSound, StopTaggedSounds},
    event::Event,
};
use red4ext_rs::{conv::ClassType, types::CName};

pub enum AnyEvent {
    PlaySound(PlaySound),
    StopSound(StopSound),
    StopTaggedSounds(StopTaggedSounds),
    SoundParameter(SoundParameter),
    Unknown(Event),
}

impl std::fmt::Display for AnyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyEvent::PlaySound(event) => write!(f, "{}", event),
            AnyEvent::StopSound(event) => write!(f, "{}", event),
            AnyEvent::StopTaggedSounds(event) => write!(f, "{}", event),
            AnyEvent::SoundParameter(event) => write!(f, "{}", event),
            AnyEvent::Unknown(event) => write!(f, "{}", event),
        }
    }
}

/// 0x141D1FB8C
pub fn print_1(event: AnyEvent) {
    red4ext_rs::info!("from 0x141D1FB8C:\n{}", event);
}
/// 0x141D1FB98
pub fn print_2(event: AnyEvent) {
    red4ext_rs::info!("from 0x141D1FB98:\n{}", event);
}
/// 0x141D1FBB0
pub fn print_3(event: AnyEvent) {
    red4ext_rs::info!("from 0x141D1FBB0:\n{}", event);
}
/// 0x141D1FBA4
pub fn print_4(event: AnyEvent) {
    red4ext_rs::info!("from 0x141D1FBA4:\n{}", event);
}
/// 0x141D1FBBC
pub fn print_5(event: AnyEvent) {
    red4ext_rs::info!("from 0x141D1FBBC:\n{}", event);
}

#[derive(NativeHandler)]
#[hook(
    offset = ON_PLAYSOUND_EVENT,
    event = "self::AnyEvent",
    detour = "print_1",
    handler = "self::from_ptr"
)]
pub struct HookgameaudioeventsSound1;
#[derive(NativeHandler)]
#[hook(
    offset = ON_STOPSOUND_EVENT,
    event = "self::AnyEvent",
    detour = "print_2",
    handler = "self::from_ptr"
)]
pub struct HookgameaudioeventsSound2;
#[derive(NativeHandler)]
#[hook(
    offset = ON_SOUNDSWITCH_EVENT,
    event = "self::AnyEvent",
    detour = "print_3",
    handler = "self::from_ptr"
)]
pub struct HookgameaudioeventsSound3;
#[derive(NativeHandler)]
#[hook(
    offset = ON_STOPTAGGEDSOUNDS_EVENT,
    event = "self::AnyEvent",
    detour = "print_4",
    handler = "self::from_ptr"
)]
pub struct HookgameaudioeventsSound4;
#[derive(NativeHandler)]
#[hook(
    offset = ON_SOUNDPARAMETER_EVENT,
    event = "self::AnyEvent",
    detour = "print_5",
    handler = "self::from_ptr"
)]
pub struct HookgameaudioeventsSound5;

fn from_ptr(event: usize) -> AnyEvent {
    let mut inner_event = Event::from_memory(event);
    let ptr_iscriptable = &mut inner_event as *mut _ as *mut red4ext_rs::ffi::IScriptable;
    let ptr_class: *mut red4ext_rs::ffi::CClass =
        unsafe { Pin::new_unchecked(&mut *ptr_iscriptable).get_class() };
    if let Some(name) = red4ext_rs::rtti::Rtti::type_name_of(
        ptr_class as *const _ as *const red4ext_rs::ffi::CBaseRttiType,
    ) {
        if name == CName::new(PlaySound::NATIVE_NAME) {
            return AnyEvent::PlaySound(PlaySound::from_memory(event));
        } else if name == CName::new(StopSound::NATIVE_NAME) {
            return AnyEvent::StopSound(StopSound::from_memory(event));
        } else if name == CName::new(StopTaggedSounds::NATIVE_NAME) {
            return AnyEvent::StopTaggedSounds(StopTaggedSounds::from_memory(event));
        } else if name == CName::new(SoundParameter::NATIVE_NAME) {
            return AnyEvent::SoundParameter(SoundParameter::from_memory(event));
        }
    } else {
        red4ext_rs::error!("unable to determine event type");
    }
    AnyEvent::Unknown(inner_event)
}
