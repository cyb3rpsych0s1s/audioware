use audioware_bank::Banks;
use red4ext_rs::{
    hashes, hooks, log,
    types::{CName, EntityId, IScriptable, StackFrame},
    PluginOps, SdkEnv, VoidPtr,
};

use crate::Audioware;

hooks! {
   static HOOK: fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = hashes::resolve(super::offsets::SWITCH);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(env, "attached hook for AudioSystem.Switch");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    i: *mut IScriptable,
    f: *mut StackFrame,
    a3: VoidPtr,
    a4: VoidPtr,
    cb: unsafe extern "C" fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr),
) {
    let frame = &mut *f;
    let state = frame.state();

    let switch_name: CName = StackFrame::get_arg(frame);
    let switch_value: CName = StackFrame::get_arg(frame);
    let entity_id: EntityId = StackFrame::get_arg(frame);
    let emitter_name: CName = StackFrame::get_arg(frame);

    if Banks::exists(&switch_name) || Banks::exists(&switch_value) {
        let env = Audioware::env();
        log::info!(
            env,
            "AudioSystem.Switch: intercepted {switch_name}/{switch_value}"
        );
    } else {
        frame.rewind(state);
        cb(i, f, a3, a4);
    }
}
