use red4ext_rs::{addr_hashes, hooks, types::IScriptable, SdkEnv};

use crate::types::Dive;

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut Dive) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(crate::hooks::offsets::DIVE_EVENT_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::lifecycle!("attached hook for Dive event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut Dive,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut Dive),
) {
    if !a2.is_null() {
        crate::utils::lifecycle!("intercepted Dive",);
    } else {
        crate::utils::lifecycle!("intercepted Dive (null)");
    }

    cb(a1, a2);
}
