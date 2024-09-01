use red4ext_rs::{addr_hashes, hooks, types::IScriptable, SdkEnv};

use crate::types::{EmitterEvent, StopSoundOnEmitter};

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut StopSoundOnEmitter) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(crate::hooks::offsets::STOP_SOUND_ON_EMITTER_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::lifecycle!("attached hook for StopSoundOnEmitter event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut StopSoundOnEmitter,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut StopSoundOnEmitter),
) {
    if !a2.is_null() {
        let event = unsafe { &*a2 };
        let &StopSoundOnEmitter { sound_name, .. } = event;
        let &EmitterEvent { emitter_name, .. } = event.as_ref();
        crate::utils::lifecycle!(
            "intercepted StopSoundOnEmitter:
- base.emitter_name: {emitter_name}
- sound_name: {sound_name}",
        );
    } else {
        crate::utils::lifecycle!("intercepted StopSoundOnEmitter (null)");
    }

    cb(a1, a2);
}
