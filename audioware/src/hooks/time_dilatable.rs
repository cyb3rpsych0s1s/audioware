use std::mem;

use red4ext_rs::types::{CName, IScriptable, StackArgsState, StackFrame};

use crate::{utils::lifecycle, Entity};

use super::NativeFunc;

#[allow(non_camel_case_types)]
pub struct SetIndividualTimeDilation;

impl NativeFunc<{ super::offsets::TIMEDILATABLE_SETINDIVIDUALTIMEDILATION }>
    for SetIndividualTimeDilation
{
    fn detour(
        this: *mut IScriptable,
        frame: &mut StackFrame,
        state: StackArgsState,
    ) -> Option<StackArgsState> {
        let x = unsafe { &*this };
        let x = unsafe { std::mem::transmute::<&IScriptable, &Entity>(x) };

        let reason: CName = unsafe { StackFrame::get_arg(frame) };
        let dilation: f32 = unsafe { StackFrame::get_arg(frame) };
        let duration: f32 = unsafe { StackFrame::get_arg(frame) };
        let ease_in_curve: CName = unsafe { StackFrame::get_arg(frame) };
        let ease_out_curve: CName = unsafe { StackFrame::get_arg(frame) };
        let ignore_global_dilation: bool = unsafe { StackFrame::get_arg(frame) };
        let use_real_time: bool = unsafe { StackFrame::get_arg(frame) };

        lifecycle!(
            "set individual time dilation {:?}:
- reason: {reason}
- dilation: {dilation}
- duration: {duration}
- ease_in_curve: {ease_in_curve}
- ease_out_curve: {ease_out_curve}
- ignore_global_dilation: {ignore_global_dilation}
- use_real_time: {use_real_time}",
            x.entity_id
        );
        Some(state)
    }

    #[cfg(debug_assertions)]
    fn name() -> &'static str {
        "TimeDilatable::SetIndividualTimeDilation"
    }

    fn storage() -> super::NativeFuncHook {
        todo!()
    }
}

#[allow(non_camel_case_types)]
pub struct UnsetIndividualTimeDilation;

impl NativeFunc<{ super::offsets::TIMEDILATABLE_UNSETINDIVIDUALTIMEDILATION }>
    for UnsetIndividualTimeDilation
{
    fn detour(
        this: *mut IScriptable,
        frame: &mut StackFrame,
        state: StackArgsState,
    ) -> Option<StackArgsState> {
        let x = unsafe { &*this };
        let x = unsafe { mem::transmute::<&IScriptable, &Entity>(x) };

        let ease_out_curve: CName = unsafe { StackFrame::get_arg(frame) };

        lifecycle!(
            "unset individual time dilation {:?}:
- ease_out_curve: {ease_out_curve}",
            x.entity_id
        );
        Some(state)
    }

    #[cfg(debug_assertions)]
    fn name() -> &'static str {
        "TimeDilatable::UnsetIndividualTimeDilation"
    }
}
