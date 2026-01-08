use red4ext_rs::{
    VoidPtr,
    types::{IScriptable, Ref, StackFrame},
};

use crate::{Event, abi::DynamicSoundEvent, attach_native_func, utils::intercept};

attach_native_func!(
    "gameuiGameSystemUI::QueueEvent",
    super::offsets::UISYSTEM_QUEUE_EVENT
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

        let mut passthru = true;
        if event.is_a::<DynamicSoundEvent>() {
            let dynamic: Ref<DynamicSoundEvent> = std::mem::transmute(event);
            if let Some(dynamic) = dynamic.fields() {
                passthru = !dynamic.enqueue_and_play(None, None);
                intercept!(
                    "gameuiGameSystemUI::QueueEvent for DynamicSoundEvent ({})",
                    dynamic.name.get()
                );
            }
        }

        frame.restore_args(state);
        if passthru {
            cb(i, frame as *mut _, a3, a4);
        }
    }
}
