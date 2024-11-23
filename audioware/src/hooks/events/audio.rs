use red4ext_rs::types::IScriptable;

use crate::{attach_native_event, AudioEvent};

attach_native_event!(
    "entAudioEvent",
    super::super::offsets::AUDIO_EVENT,
    crate::AudioEvent
);

unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut AudioEvent,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut AudioEvent),
) {
    let event = &*a2;
    let AudioEvent {
        event_name,
        emitter_name,
        name_data,
        float_data,
        event_type,
        event_flags,
        ..
    } = event;
    crate::utils::lifecycle!(
        "intercepted entAudioEvent:
- event_name: {event_name}
- emitter_name: {emitter_name}
- name_data: {name_data}
- float_data: {float_data}
- event_type: {event_type}
- event_flags: {event_flags}"
    );
    cb(a1, a2);
}
