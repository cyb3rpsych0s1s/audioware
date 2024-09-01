use red4ext_rs::{addr_hashes, hooks, types::IScriptable, SdkEnv};

use crate::types::VoicePlayedEvent;

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut VoicePlayedEvent) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(crate::hooks::offsets::VOICE_PLAYED_EVENT_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::lifecycle!("attached hook for VoicePlayedEvent event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut VoicePlayedEvent,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut VoicePlayedEvent),
) {
    if !a2.is_null() {
        let &VoicePlayedEvent {
            event_name,
            grunt_type,
            is_v,
            ..
        } = unsafe { &*a2 };
        crate::utils::lifecycle!(
            "intercepted VoicePlayedEvent:
- event_name: {event_name}
- grunt_type: {grunt_type}
- is_v: {is_v}",
        );
    } else {
        crate::utils::lifecycle!("intercepted VoicePlayedEvent (null)");
    }

    cb(a1, a2);
}
