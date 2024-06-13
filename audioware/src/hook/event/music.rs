use super::super::address::ON_MUSIC_EVENT;
use audioware_macros::NativeHandler;
use audioware_mem::FromMemory;
use audioware_sys::interop::audio::MusicEvent;
use red4ext_rs::conv::ClassType;

pub fn print_music_event(event: MusicEvent) {
    let MusicEvent { event_name, .. } = event;
    audioware_core::dbg(format!(
        "intercepted {} ({}): {}",
        MusicEvent::NAME,
        MusicEvent::NATIVE_NAME,
        event_name
    ));
}

#[derive(NativeHandler)]
#[hook(
    offset = ON_MUSIC_EVENT,
    event = "audioware_sys::interop::audio::MusicEvent",
    detour = "print_music_event"
)]
pub struct HookgameaudioeventsMusicEvent;
