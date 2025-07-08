use red4ext_rs::ScriptClass;
use red4ext_rs::types::IScriptable;

use crate::{VehicleAudioEvent, attach_native_event};

attach_native_event!(
    super::super::offsets::VEHICLE_AUDIO_EVENT,
    crate::VehicleAudioEvent
);

unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut VehicleAudioEvent,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut VehicleAudioEvent),
) {
    unsafe {
        let event = &*a2;
        let VehicleAudioEvent { action, .. } = event;
        crate::utils::lifecycle!(
            "intercepted {}:
- action {action}",
            VehicleAudioEvent::NAME
        );
        cb(a1, a2);
    }
}
