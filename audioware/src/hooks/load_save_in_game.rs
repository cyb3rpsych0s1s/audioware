use red4ext_rs::{
    addr_hashes, hooks, log,
    types::{IScriptable, StackFrame},
    PluginOps, SdkEnv, VoidPtr,
};

use crate::{engine::Engine, Audioware};

hooks! {
   static HOOK: fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::LOAD_SAVE_IN_GAME);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(
        env,
        "attached hook for gameuiSaveHandlingController.LoadSaveInGame/LoadModdedSave"
    );
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    i: *mut IScriptable,
    f: *mut StackFrame,
    a3: VoidPtr,
    a4: VoidPtr,
    cb: unsafe extern "C" fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr),
) {
    let env = Audioware::env();
    log::info!(
        env,
        "gameuiSaveHandlingController.LoadSaveInGame/LoadModdedSave: called"
    );
    Engine::shutdown();
    cb(i, f, a3, a4);
}
