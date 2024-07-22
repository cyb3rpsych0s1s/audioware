use audioware_bank::Banks;
use red4ext_rs::{
    addr_hashes, hooks, log,
    types::{CName, EntityId, GameEngine, IScriptable, Opt, StackFrame},
    PluginOps, RttiSystem, ScriptClass, SdkEnv, VoidPtr,
};

use crate::{
    types::{AsAudioSystem, AudioSystem},
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
    let entity_id: EntityId = StackFrame::get_arg(frame);
    let emitter_name: CName = StackFrame::get_arg(frame);

    let prev = Banks::exists(&switch_name);
    let next = Banks::exists(&switch_value);

    if prev || next {
        let rtti = RttiSystem::get();
        let class = rtti.get_class(CName::new(AudioSystem::NAME)).unwrap();
        let engine = GameEngine::get();
        let game = engine.game_instance();
        let system = game
            .get_system(class.as_type())
            .cast::<AudioSystem>()
            .unwrap();
        let entity_id = if entity_id == EntityId::default() {
            Opt::Default
        } else {
            Opt::NonDefault(entity_id)
        };
        let emitter_name = if emitter_name == CName::default() {
            Opt::Default
        } else {
            Opt::NonDefault(emitter_name)
        };
        if !prev {
            system.stop(switch_name, entity_id, emitter_name);
        }
        // TODO: kira stop/play
        if !next {
            system.play(switch_value, entity_id, emitter_name);
        }
        let env = Audioware::env();
        log::info!(
            env,
            "AudioSystem.Switch: intercepted {switch_name}/{switch_value}"
        );
    } else {
        frame.restore_args(state);
        cb(i, f, a3, a4);
    }
}
