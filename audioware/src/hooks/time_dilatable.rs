use std::{mem, ops::Not};

use red4ext_rs::types::{CName, IScriptable, StackFrame};

use crate::{attach_native_func, engine::queue::notify, utils::intercept, Entity};

use red4ext_rs::{SdkEnv, VoidPtr};

pub fn attach_hooks(env: &SdkEnv) {
    attach_hook_set(env);
    attach_hook_unset(env);
}

// Set time dilation on NPCs.
attach_native_func!(
    "TimeDilatable::SetIndividualTimeDilation",
    super::offsets::TIMEDILATABLE_SETINDIVIDUALTIMEDILATION,
    HOOK_SET,
    attach_hook_set,
    detour_set
);

// Unset time dilation on NPCs.
attach_native_func!(
    "TimeDilatable::UnsetIndividualTimeDilation",
    super::offsets::TIMEDILATABLE_UNSETINDIVIDUALTIMEDILATION,
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

    intercept!(
        "TimeDilatable::SetIndividualTimeDilation {x:?}:
- reason: {reason}
- dilation: {dilation}
- duration: {duration}
- ease_in_curve: {ease_in_curve}
- ease_out_curve: {ease_out_curve}
- ignore_global_dilation: {ignore_global_dilation}
- use_real_time: {use_real_time}",
    );
    if let Some(entity_id) = x {
        notify(crate::abi::lifecycle::Lifecycle::SetEmitterDilation {
            reason,
            entity_id,
            value: dilation,
            ease_in_curve,
        });
    }
    cb(i, f, a3, a4);
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

    let x = i
        .is_null()
        .not()
        .then_some(&*i)
        .map(|x| mem::transmute::<&IScriptable, &Entity>(x))
        .map(|x| x.entity_id);

    let ease_out_curve: CName = unsafe { StackFrame::get_arg(frame) };
    frame.restore_args(state);

    intercept!(
        "TimeDilatable::UnsetIndividualTimeDilation {x:?}:
- ease_out_curve: {ease_out_curve}",
    );
    if let Some(entity_id) = x {
        notify(crate::abi::lifecycle::Lifecycle::UnsetEmitterDilation {
            entity_id,
            ease_out_curve,
        });
    }
    cb(i, f, a3, a4);
}
