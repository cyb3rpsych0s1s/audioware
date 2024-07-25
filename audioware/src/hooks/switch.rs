use audioware_bank::Banks;
use red4ext_rs::{
    addr_hashes, hooks, log,
    types::{CName, EntityId, GameInstance, IScriptable, Opt, StackFrame},
    PluginOps, SdkEnv, VoidPtr,
};

use crate::{
    engine::{effects::SMOOTHLY, Engine},
    types::{AsAudioSystem, AsGameInstance},
    Audioware,
};

hooks! {
   static HOOK: fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::SWITCH);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(env, "attached hook for AudioSystem.Switch");
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

    let switch_name: CName = StackFrame::get_arg(frame);
    let switch_value: CName = StackFrame::get_arg(frame);
    let entity_id: Opt<EntityId> = StackFrame::get_arg(frame);
    let emitter_name: Opt<CName> = StackFrame::get_arg(frame);

    let prev = Banks::exists(&switch_name);
    let next = Banks::exists(&switch_value);

    if prev || next {
        let env = Audioware::env();
        log::info!(
            env,
            "AudioSystem.Switch: intercepted {switch_name}/{switch_value}"
        );

        let system = GameInstance::get_audio_system();

        if prev {
            match entity_id.into_option() {
                Some(x) => Engine::stop_by_cname_for_entity(&switch_name, &x, Some(SMOOTHLY)),
                None => Engine::stop_by_cname(&switch_name, Some(SMOOTHLY)),
            };
        } else {
            system.stop(switch_name, entity_id, emitter_name);
        }

        if next {
            Engine::play(
                switch_value,
                entity_id.into_option(),
                emitter_name.into_option(),
                None,
                Some(SMOOTHLY),
            );
        } else {
            system.play(switch_value, entity_id, emitter_name);
        }
    } else {
        frame.restore_args(state);
        cb(i, f, a3, a4);
    }
}
