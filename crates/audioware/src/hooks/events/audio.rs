use red4ext_rs::ScriptClass;
use red4ext_rs::types::IScriptable;

use crate::{AudioEvent, attach_native_event};

attach_native_event!(super::super::offsets::AUDIO_EVENT, crate::AudioEvent);

unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut AudioEvent,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut AudioEvent),
) {
    unsafe {
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
        // if *event_type == EventActionType::SetEntityName
        //     || *event_type == EventActionType::AddContainerStreamingPrefetch
        //     || *event_type == EventActionType::RemoveContainerStreamingPrefetch
        //     || *event_type == EventActionType::PlayExternal
        //     || *event_type == EventActionType::SetAppearanceName
        // {
        crate::utils::lifecycle!(
            "intercepted {}:
    - event_name: {event_name}
    - emitter_name: {emitter_name}
    - name_data: {name_data}
    - float_data: {float_data}
    - event_type: {event_type}
    - event_flags: {event_flags}",
            AudioEvent::NAME
        );
        // }
        cb(a1, a2);
    }
}
