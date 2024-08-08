use red4ext_rs::{addr_hashes, hooks, log, types::IScriptable, PluginOps, SdkEnv};

use crate::{types::Surface, Audioware};

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut Surface) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(crate::hooks::offsets::SURFACE_EVENT_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(env, "attached hook for Surface event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut Surface,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut Surface),
) {
    if !a2.is_null() {
        log::info!(Audioware::env(), "intercepted Surface",);
    } else {
        log::info!(Audioware::env(), "intercepted Surface (null)");
    }

    cb(a1, a2);
}
