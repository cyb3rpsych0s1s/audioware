use red4ext_rs::{addr_hashes, hooks, log, types::IScriptable, PluginOps, SdkEnv};

use crate::{types::AudioEvent, Audioware};

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut AudioEvent) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(crate::hooks::offsets::AUDIO_EVENT_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(env, "attached hook for AudioEvent event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut AudioEvent,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut AudioEvent),
) {
    if !a2.is_null() {
        let &AudioEvent {
            event_name,
            emitter_name,
            name_data,
            float_data,
            event_type,
            event_flags,
            ..
        } = unsafe { &*a2 };
        log::info!(
            Audioware::env(),
            "intercepted AudioEvent:
- event_name: {event_name}
- emitter_name: {emitter_name}
- name_data: {name_data}
- float_data: {float_data}
- event_type: {event_type}
- event_flags: {event_flags}",
        );
    } else {
        log::info!(Audioware::env(), "intercepted AudioEvent (null)");
    }

    cb(a1, a2);
}
