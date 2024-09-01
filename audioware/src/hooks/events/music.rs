use red4ext_rs::{addr_hashes, hooks, types::IScriptable, SdkEnv};

use crate::types::MusicEvent;

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut MusicEvent) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(crate::hooks::offsets::MUSIC_EVENT_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::lifecycle!("attached hook for MusicEvent event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut MusicEvent,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut MusicEvent),
) {
    if !a2.is_null() {
        let &MusicEvent { event_name, .. } = unsafe { &*a2 };
        crate::utils::lifecycle!(
            "intercepted MusicEvent:
- event_name: {event_name}",
        );
    } else {
        crate::utils::lifecycle!("intercepted MusicEvent (null)");
    }

    cb(a1, a2);
}
