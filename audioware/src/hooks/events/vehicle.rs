use red4ext_rs::{addr_hashes, hooks, types::IScriptable, SdkEnv};

use crate::types::VehicleAudioEvent;

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut VehicleAudioEvent) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(crate::hooks::offsets::VEHICLE_AUDIO_EVENT_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::lifecycle!("attached hook for VehicleAudioEvent event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut VehicleAudioEvent,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut VehicleAudioEvent),
) {
    if !a2.is_null() {
        let &VehicleAudioEvent { action, .. } = unsafe { &*a2 };
        crate::utils::lifecycle!(
            "intercepted VehicleAudioEvent:
- action: {action}",
        );
    } else {
        crate::utils::lifecycle!("intercepted VehicleAudioEvent (null)");
    }

    cb(a1, a2);
}
