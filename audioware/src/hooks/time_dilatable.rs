use std::{mem, ops::Not};

use red4ext_rs::{
    hooks,
    types::{CName, IScriptable, StackFrame},
};

use crate::{utils::lifecycle, Entity};

use red4ext_rs::{addr_hashes, SdkEnv, VoidPtr};

hooks! {
   static HOOK_SET: fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook_set(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::TIMEDILATABLE_SETINDIVIDUALTIMEDILATION);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK_SET, addr, detour_set) };
    #[cfg(debug_assertions)]
    crate::utils::lifecycle!("attached hook for TimeDilatable::SetIndividualTimeDilation");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour_set(
    i: *mut IScriptable,
    f: *mut StackFrame,
    a3: VoidPtr,
    a4: VoidPtr,
    cb: unsafe extern "C" fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr),
) {
    let frame = &mut *f;
    let state = frame.args_state();

    let x = i
        .is_null()
        .not()
        .then_some(&*i)
        .map(|x| mem::transmute::<&IScriptable, &Entity>(x))
        .map(|x| x.entity_id);

    let reason: CName = unsafe { StackFrame::get_arg(frame) };
    let dilation: f32 = unsafe { StackFrame::get_arg(frame) };
    let duration: f32 = unsafe { StackFrame::get_arg(frame) };
    let ease_in_curve: CName = unsafe { StackFrame::get_arg(frame) };
    let ease_out_curve: CName = unsafe { StackFrame::get_arg(frame) };
    let ignore_global_dilation: bool = unsafe { StackFrame::get_arg(frame) };
    let use_real_time: bool = unsafe { StackFrame::get_arg(frame) };
    frame.restore_args(state);

    lifecycle!(
        "set individual time dilation {x:?}:
- ease_out_curve: {ease_out_curve}",
    );
    cb(i, f, a3, a4);
}

hooks! {
   static HOOK_UNSET: fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook_unset(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::TIMEDILATABLE_UNSETINDIVIDUALTIMEDILATION);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK_UNSET, addr, detour_unset) };
    #[cfg(debug_assertions)]
    crate::utils::lifecycle!("attached hook for TimeDilatable::UnsetIndividualTimeDilation");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour_unset(
    i: *mut IScriptable,
    f: *mut StackFrame,
    a3: VoidPtr,
    a4: VoidPtr,
    cb: unsafe extern "C" fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr),
) {
    let frame = &mut *f;
    let state = frame.args_state();

    let x = i
        .is_null()
        .not()
        .then_some(&*i)
        .map(|x| mem::transmute::<&IScriptable, &Entity>(x))
        .map(|x| x.entity_id);

    let ease_out_curve: CName = unsafe { StackFrame::get_arg(frame) };
    frame.restore_args(state);

    lifecycle!(
        "unset individual time dilation {x:?}:
- ease_out_curve: {ease_out_curve}",
    );
    cb(i, f, a3, a4);
}
