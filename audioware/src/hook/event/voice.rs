use super::super::address::{ON_VOICEPLAYED_EVENT, ON_VOICE_EVENT};
use audioware_core::audioware_dbg;
use audioware_macros::NativeHandler;
use audioware_mem::FromMemory;
use audioware_sys::interop::audio::{VoiceEvent, VoicePlayedEvent};
use red4ext_rs::conv::ClassType;

pub fn print_event(event: VoiceEvent) {
    let VoiceEvent {
        event_name,
        grunt_type,
        grunt_interrupt_mode,
        is_v,
        ..
    } = event;
    audioware_dbg!(
        "intercepted {} ({}): {}, {}, {}, {}",
        VoiceEvent::NAME,
        VoiceEvent::NATIVE_NAME,
        event_name,
        grunt_type,
        grunt_interrupt_mode,
        is_v
    );
}

pub fn print_event_played(event: VoicePlayedEvent) {
    let VoicePlayedEvent {
        event_name,
        grunt_type,
        is_v,
        ..
    } = event;
    audioware_dbg!(
        "intercepted {} ({}): {}, {}, {}",
        VoicePlayedEvent::NAME,
        VoicePlayedEvent::NATIVE_NAME,
        event_name,
        grunt_type,
        is_v
    );
}

#[derive(NativeHandler)]
#[hook(
    offset = ON_VOICE_EVENT,
    event = "audioware_sys::interop::audio::VoiceEvent",
    detour = "print_event"
)]
pub struct HookgameaudioeventsVoiceEvent;

#[derive(NativeHandler)]
#[hook(
    offset = ON_VOICEPLAYED_EVENT,
    event = "audioware_sys::interop::audio::VoicePlayedEvent",
    detour = "print_event_played"
)]
pub struct HookgameaudioeventsVoicePlayedEvent;
