use red4ext_rs::{
    addr_hashes, hooks, log,
    types::{CName, IScriptable, StackFrame},
    PluginOps, SdkEnv, VoidPtr,
};

use crate::Audioware;

hooks! {
   static HOOK: fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::PARAMETER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(env, "attached hook for AudioSystem.Parameter");
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

    let parameter_name: CName = StackFrame::get_arg(frame);
    let parameter_value: f32 = StackFrame::get_arg(frame);

    let env = Audioware::env();
    log::info!(
        env,
        "AudioSystem.GlobalParameter: called {parameter_name} {parameter_value}"
    );
    frame.restore_args(state);
    cb(i, f, a3, a4);
}
