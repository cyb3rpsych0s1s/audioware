use red4ext_rs::{addr_hashes, hooks, log, types::IScriptable, PluginOps, SdkEnv};

use crate::{types::StopDialogLine, Audioware};

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut StopDialogLine) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::STOP_DIALOG_LINE_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(env, "attached hook for StopDialogLine event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut StopDialogLine,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut StopDialogLine),
) {
    if !a2.is_null() {
        let &StopDialogLine { .. } = unsafe { &*a2 };
        log::info!(
            Audioware::env(),
            "intercepted StopDialogLine:
- ",
        );
    } else {
        log::info!(Audioware::env(), "intercepted StopDialogLine (null)");
    }

    cb(a1, a2);
}
