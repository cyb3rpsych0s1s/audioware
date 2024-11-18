use red4ext_rs::{
    addr_hashes, hooks,
    types::{CName, IScriptable, StackFrame},
    SdkEnv, VoidPtr,
};

hooks! {
   static HOOK: fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::TIMESYSTEM_SETTIMEDILATION);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    #[cfg(debug_assertions)]
    crate::utils::lifecycle!("attached hook for TimeSystem::SetTimeDilation");
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

    let reason: CName = unsafe { StackFrame::get_arg(frame) };
    let dilation: f32 = unsafe { StackFrame::get_arg(frame) };
    let duration: f32 = unsafe { StackFrame::get_arg(frame) };
    let ease_in_curve: CName = unsafe { StackFrame::get_arg(frame) };
    let ease_out_curve: CName = unsafe { StackFrame::get_arg(frame) };
    // let _listener: Ref<IScriptable> = unsafe { StackFrame::get_arg(frame) };
    frame.restore_args(state);

    crate::utils::lifecycle!(
        "set dilation time on time system:
    - reason: {reason}
    - dilation: {dilation}
    - duration: {duration}
    - ease_in_curve: {ease_in_curve}
    - ease_out_curve: {ease_out_curve}",
    );
    cb(i, frame as *mut _, a3, a4);
}
