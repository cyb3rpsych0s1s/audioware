use red4ext_rs::types::{CName, IScriptable, Ref, StackArgsState, StackFrame};

use crate::utils::lifecycle;

use super::NativeFunc;

// 140A46EE4
#[allow(non_camel_case_types)]
pub struct SetTimeDilation;

impl NativeFunc<{ super::offsets::TIMESYSTEM_SETTIMEDILATION }> for SetTimeDilation {
    fn detour(
        _: *mut IScriptable,
        frame: &mut StackFrame,
        state: StackArgsState,
    ) -> Option<StackArgsState> {
        let reason: CName = unsafe { StackFrame::get_arg(frame) };
        let dilation: f32 = unsafe { StackFrame::get_arg(frame) };
        let duration: f32 = unsafe { StackFrame::get_arg(frame) };
        let ease_in_curve: CName = unsafe { StackFrame::get_arg(frame) };
        let ease_out_curve: CName = unsafe { StackFrame::get_arg(frame) };
        let _: Ref<IScriptable> = unsafe { StackFrame::get_arg(frame) };

        lifecycle!(
            "set dilation time:
- reason: {reason}
- dilation: {dilation}
- duration: {duration}
- ease_in_curve: {ease_in_curve}
- ease_out_curve: {ease_out_curve}",
        );
        Some(state)
    }

    #[cfg(debug_assertions)]
    fn name() -> &'static str {
        "TimeSystem::SetTimeDilation"
    }
}
