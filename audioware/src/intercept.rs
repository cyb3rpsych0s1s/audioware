use audioware_macros::NativeHandler;
use audioware_mem::FromMemory;
use audioware_sys::interop::audio::AudioEvent;

use crate::addresses::ON_ENTAUDIOEVENT;

pub fn print_ent_audio_event(event: AudioEvent) {
    use red4ext_rs::conv::ClassType;
    let AudioEvent {
        event_name,
        emitter_name,
        name_data,
        float_data,
        event_type,
        event_flags,
        ..
    } = event;
    red4ext_rs::info!(
        "intercepted {} ({}):\nevent_name: {}\nemitter_name: {}\nname_data: {}\nfloat_data: {}\nevent_type: {}\nevent_flags: {}\n",
        AudioEvent::NAME,
        AudioEvent::NATIVE_NAME,
        event_name,
        emitter_name,
        name_data,
        float_data,
        event_type,
        event_flags
    );
}

#[derive(NativeHandler)]
#[hook(
    offset = ON_ENTAUDIOEVENT,
    event = "audioware_sys::interop::audio::AudioEvent",
    detour = "print_ent_audio_event"
)]
pub struct HookentAudioEvent;
