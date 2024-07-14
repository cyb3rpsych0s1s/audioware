use audioware_bank::Banks;
use red4ext_rs::{
    hashes, hooks, log,
    types::{CName, EntityId, IScriptable, StackFrame},
    PluginOps, SdkEnv, VoidPtr,
};

use crate::Audioware;

const OFFSET: u32 = 0xCDB11D0E; // 0x140974F58

hooks! {
   static HOOK: fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub(super) fn attach_hook_audiosystem_play(env: &SdkEnv) {
    let addr = hashes::resolve(OFFSET);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(env, "attached hook for AudioSystem.Play");
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

    let sound_name: CName = StackFrame::get_arg(frame);
    let emitter_id: EntityId = StackFrame::get_arg(frame);
    let emitter_name: CName = StackFrame::get_arg(frame);

    if Banks::exists(&sound_name) {
        let env = Audioware::env();
        log::info!(env, "AudioSystem.Play: intercepted {sound_name}");
    } else {
        cb(i, f, a3, a4);
    }
}
