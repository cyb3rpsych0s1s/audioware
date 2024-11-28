use red4ext_rs::types::IScriptable;
use red4ext_rs::ScriptClass;

use crate::{attach_native_event, VehicleAudioEvent};

attach_native_event!(
    super::super::offsets::VEHICLE_AUDIO_EVENT,
    crate::VehicleAudioEvent
);

unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut VehicleAudioEvent,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut VehicleAudioEvent),
) {
    let event = &*a2;
    let VehicleAudioEvent { action, .. } = event;
    crate::utils::lifecycle!(
        "intercepted {}:
- action {action}",
        VehicleAudioEvent::NAME
    );
    cb(a1, a2);
}
