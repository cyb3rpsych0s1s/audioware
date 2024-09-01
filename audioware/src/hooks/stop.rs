use audioware_bank::Banks;
use red4ext_rs::{
    addr_hashes, hooks,
    types::{CName, EntityId, IScriptable, StackFrame},
    PluginOps, SdkEnv, VoidPtr,
};

use crate::{engine::Engine, Audioware};

hooks! {
   static HOOK: fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::STOP);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::lifecycle!("attached hook for AudioSystem.Stop");
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

    let event_name: CName = StackFrame::get_arg(frame);
    let entity_id: EntityId = StackFrame::get_arg(frame);
    let emitter_name: CName = StackFrame::get_arg(frame);

    if Banks::exists(&event_name) {
        let env = Audioware::env();
        crate::utils::lifecycle!("AudioSystem.Stop: intercepted {event_name}");

        Engine::send(crate::engine::commands::Command::StopVanilla {
            event_name,
            entity_id: entity_id.into(),
            emitter_name: emitter_name.into(),
        });
    } else {
        frame.restore_args(state);
        cb(i, f, a3, a4);
    }
}
