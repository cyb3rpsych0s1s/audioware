use red4ext_rs::{
    types::{CName, IScriptable, Ref, StackFrame},
    SdkEnv, VoidPtr,
};

use crate::{attach_hook, utils::intercept};

pub fn attach_hooks(env: &SdkEnv) {
    attach_hook_set(env);
    attach_hook_unset(env);
}

attach_hook!(
    "TimeSystem::SetTimeDilation",
    super::offsets::TIMESYSTEM_SETTIMEDILATION,
    HOOK_SET,
    attach_hook_set,
    detour_set
);

attach_hook!(
    "TimeSystem::UnsetTimeDilation",
    super::offsets::TIMESYSTEM_UNSETTIMEDILATION,
    HOOK_UNSET,
    attach_hook_unset,
    detour_unset
);

unsafe extern "C" fn detour_set(
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
    let _listener: Ref<IScriptable> = unsafe { StackFrame::get_arg(frame) };
    frame.restore_args(state);

    intercept!(
        "TimeSystem::SetTimeDilation:
    - reason: {reason}
    - dilation: {dilation}
    - duration: {duration}
    - ease_in_curve: {ease_in_curve}
    - ease_out_curve: {ease_out_curve}",
    );
    cb(i, frame as *mut _, a3, a4);
}

unsafe extern "C" fn detour_unset(
    i: *mut IScriptable,
    f: *mut StackFrame,
    a3: VoidPtr,
    a4: VoidPtr,
    cb: unsafe extern "C" fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr),
) {
    let frame = &mut *f;
    let state = frame.args_state();

    let reason: CName = unsafe { StackFrame::get_arg(frame) };
    let ease_out_curve: CName = unsafe { StackFrame::get_arg(frame) };
    frame.restore_args(state);

    intercept!(
        "TimeSystem::UnsetTimeDilation:
    - reason: {reason}
    - ease_out_curve: {ease_out_curve}",
    );
    cb(i, frame as *mut _, a3, a4);
}
