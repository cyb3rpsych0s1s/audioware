use red4ext_rs::{
    addr_hashes, hooks,
    types::{IScriptable, Ref, StackFrame},
    SdkEnv, VoidPtr,
};
use std::mem;

use crate::types::{Entity, Event, VehicleObject};

hooks! {
   static HOOK: fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::QUEUE_EVENT);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::lifecycle!("attached hook for Entity.QueueEvent");
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

    let evt: Ref<Event> = StackFrame::get_arg(frame);

    if !i.is_null() && !evt.is_null() {
        if let Some(fields) = unsafe { evt.fields() } {
            let i = unsafe { &*i };
            let i = unsafe { mem::transmute::<&IScriptable, &Entity>(i) };
            let entity_id = i.entity_id;
            if i.as_ref().as_serializable().is_a::<VehicleObject>() {
                let iscriptable = fields.as_ref();
                let serializable = iscriptable.as_serializable();
                crate::utils::lifecycle!(
                    "Entity.QueueEvent: intercepted {} on {:?} (VehicleObject)",
                    serializable.class().name(),
                    entity_id
                );
            }
        }
    }
    frame.restore_args(state);
    cb(i, f, a3, a4);
}
