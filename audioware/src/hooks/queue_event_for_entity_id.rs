use red4ext_rs::{
    addr_hashes, hooks, log,
    types::{EntityId, IScriptable, Ref, StackFrame},
    PluginOps, SdkEnv, VoidPtr,
};

use crate::{types::Event, Audioware};

hooks! {
   static HOOK: fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::QUEUE_EVENT_FOR_ENTITY_ID);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(env, "attached hook for Entity.QueueEventForEntityID");
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

    let entity_id: EntityId = StackFrame::get_arg(frame);
    let evt: Ref<Event> = StackFrame::get_arg(frame);

    if !evt.is_null() {
        if let Some(fields) = unsafe { evt.fields() } {
            let iscriptable = fields.as_ref();
            let serializable = iscriptable.as_serializable();
            log::info!(
                Audioware::env(),
                "Entity.QueueEventForEntityID: intercepted {} on {:?}",
                serializable.class().name(),
                entity_id
            );
        }
    }
    frame.restore_args(state);
    cb(i, f, a3, a4);
}
