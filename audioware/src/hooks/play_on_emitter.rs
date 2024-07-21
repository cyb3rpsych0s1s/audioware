use audioware_bank::Banks;
use red4ext_rs::{
    addr_hashes, hooks, log,
    types::{CName, EntityId, IScriptable, StackFrame},
    PluginOps, SdkEnv, VoidPtr,
};

use crate::Audioware;

hooks! {
   static HOOK: fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::PLAY_ON_EMITTER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(env, "attached hook for AudioSystem.PlayOnEmitter");
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
    let state = frame.args_state();

    let event_name: CName = StackFrame::get_arg(frame);
    let entity_id: EntityId = StackFrame::get_arg(frame);
    let emitter_name: CName = StackFrame::get_arg(frame);

    if Banks::exists(&event_name) {
        let env = Audioware::env();
        log::info!(env, "AudioSystem.PlayOnEmitter: intercepted {event_name}");
    } else {
        frame.restore_args(state);
        cb(i, f, a3, a4);
    }
}
