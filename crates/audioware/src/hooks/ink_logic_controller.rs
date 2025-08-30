use std::mem;

use red4ext_rs::{
    ScriptClass, VoidPtr,
    types::{IScriptable, Ref, StackFrame},
};

use crate::{Event, VORequestEvt, attach_native_func, utils::intercept};

attach_native_func!(
    "inkLogicController::QueueEvent",
    super::offsets::INKLOGICCONTROLLER_QUEUE_EVENT
);

unsafe extern "C" fn detour(
    i: *mut IScriptable,
    f: *mut StackFrame,
    a3: VoidPtr,
    a4: VoidPtr,
    cb: unsafe extern "C" fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr),
) {
    unsafe {
        let frame = &mut *f;
        let state = frame.args_state();

        let event: Ref<Event> = StackFrame::get_arg(frame);
        if event.is_a::<VORequestEvt>() {
            let vo: Ref<VORequestEvt> = mem::transmute(event.clone());
            if let Some(vo) = vo.fields() {
                intercept!(
                    "inkLogicController::QueueEvent: {}
- vo_id: {:?}
- speaker_name: {}",
                    VORequestEvt::NAME,
                    vo.vo_id,
                    vo.speaker_name.to_string().as_str(),
                );
            }
        }
        frame.restore_args(state);
        cb(i, frame as *mut _, a3, a4);
    }
}
