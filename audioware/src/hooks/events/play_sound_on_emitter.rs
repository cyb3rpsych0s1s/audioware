use red4ext_rs::{addr_hashes, hooks, types::IScriptable, SdkEnv};

use crate::types::{EmitterEvent, PlaySoundOnEmitter};

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut PlaySoundOnEmitter) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(crate::hooks::offsets::PLAY_SOUND_ON_EMITTER_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::lifecycle!("attached hook for PlaySoundOnEmitter event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut PlaySoundOnEmitter,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut PlaySoundOnEmitter),
) {
    if !a2.is_null() {
        let event = unsafe { &*a2 };
        let &PlaySoundOnEmitter { event_name, .. } = event;
        let &EmitterEvent { emitter_name, .. } = event.as_ref();
        crate::utils::lifecycle!(
            "intercepted PlaySoundOnEmitter:
- base.emitter_name: {emitter_name}
- event_name: {event_name}",
        );
    } else {
        crate::utils::lifecycle!("intercepted PlaySoundOnEmitter (null)");
    }

    cb(a1, a2);
}
