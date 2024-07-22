use red4ext_rs::{addr_hashes, hooks, log, types::IScriptable, PluginOps, SdkEnv};

use crate::Audioware;

hooks! {
   static HOOK: fn(i: *mut IScriptable) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::ON_TRANSFORM_UPDATED);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(env, "attached hook for AudioSystem.Parameter");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(i: *mut IScriptable, cb: unsafe extern "C" fn(i: *mut IScriptable)) {
    let env = Audioware::env();
    log::info!(env, "GameObject.OnTransformUpdated intercepted");
    cb(i);
}
