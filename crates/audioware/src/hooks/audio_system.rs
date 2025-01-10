use kira::backend::cpal::CpalBackend;
use red4ext_rs::{
    types::{CName, EntityId, IScriptable, StackFrame},
    SdkEnv, VoidPtr,
};

use crate::{abi::command::Command, attach_native_func, engine::Engine};

pub fn attach_hooks(env: &SdkEnv) {
    attach_play(env);
    attach_stop(env);
    attach_switch(env);
}

attach_native_func!(
    "AudioSystem::Play",
    super::offsets::AUDIOSYSTEM_PLAY,
    HOOK_PLAY,
    attach_play,
    detour_play
);

attach_native_func!(
    "AudioSystem::Stop",
    super::offsets::AUDIOSYSTEM_STOP,
    HOOK_STOP,
    attach_stop,
    detour_stop
);

attach_native_func!(
    "AudioSystem::Switch",
    super::offsets::AUDIOSYSTEM_SWITCH,
    HOOK_SWITCH,
    attach_switch,
    detour_switch
);

unsafe extern "C" fn detour_play(
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

    if Engine::<CpalBackend>::exists(&event_name) {
        crate::utils::intercept!("AudioSystem.Play: intercepted {event_name}");
        crate::engine::queue::send(Command::PlayVanilla {
            event_name,
            entity_id: entity_id.into(),
            emitter_name: emitter_name.into(),
        });
    } else {
        frame.restore_args(state);
        cb(i, f, a3, a4);
    }
}

unsafe extern "C" fn detour_stop(
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

    if Engine::<CpalBackend>::exists(&event_name) {
        crate::utils::intercept!("AudioSystem.Stop: intercepted {event_name}");
        crate::engine::queue::send(Command::StopVanilla {
            event_name,
            entity_id: entity_id.into(),
            emitter_name: emitter_name.into(),
        });
    } else {
        frame.restore_args(state);
        cb(i, f, a3, a4);
    }
}

unsafe extern "C" fn detour_switch(
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

    let prev = Engine::<CpalBackend>::exists(&switch_name);
    let next = Engine::<CpalBackend>::exists(&switch_value);

    if prev || next {
        crate::utils::intercept!("AudioSystem.Switch: intercepted {switch_name}/{switch_value}");

        crate::engine::queue::send(Command::SwitchVanilla {
            switch_name,
            switch_value,
            entity_id: entity_id.into(),
            emitter_name: emitter_name.into(),
        });
    } else {
        frame.restore_args(state);
        cb(i, f, a3, a4);
    }
}
