use red4ext_rs::{
    addr_hashes, hooks,
    types::{IScriptable, StackFrame},
    SdkEnv, VoidPtr,
};

use crate::{
    engine::{commands::Lifecycle, Engine},
    utils,
};

hooks! {
   static HOOK: fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::LOAD_SAVE_IN_GAME);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    utils::lifecycle!(
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
    crate::utils::lifecycle!("gameuiSaveHandlingController.LoadSaveInGame/LoadModdedSave: called");
    Engine::notify(Lifecycle::Shutdown);
    cb(i, f, a3, a4);
}
