use red4ext_rs::SdkEnv;

pub fn attach_hooks(env: &SdkEnv) {
    queue_event::attach_hook(env);

    #[cfg(debug_assertions)]
    dispose::attach_hook(env);
}
pub fn detach_hooks(env: &SdkEnv) {
    queue_event::detach_hook(env);

    #[cfg(debug_assertions)]
    dispose::detach_hook(env);
}

#[cfg(debug_assertions)]
mod dispose {

    use std::{mem, ops::Not};

    use red4ext_rs::{
        VoidPtr,
        types::{IScriptable, StackFrame},
    };

    use crate::{Entity, attach_native_func, utils::intercept};

    attach_native_func!("Entity::Dispose", super::super::offsets::ENTITY_DISPOSE);

    unsafe extern "C" fn detour(
        i: *mut IScriptable,
        f: *mut StackFrame,
        a3: VoidPtr,
        a4: VoidPtr,
        cb: unsafe extern "C" fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr),
    ) {
        unsafe {
            let x = i
                .is_null()
                .not()
                .then(|| &*i)
                .map(|x| mem::transmute::<&IScriptable, &Entity>(x))
                .map(|x| x.entity_id);
            intercept!("called Entity::Dispose ({x:?})");
            cb(i, f, a3, a4);
        }
    }
}

mod queue_event {

    use std::mem;

    use red4ext_rs::{
        VoidPtr,
        types::{IScriptable, Ref, StackFrame},
    };

    use crate::{
        AudioEmitterComponent, Entity, Event,
        abi::{DynamicEmitterEvent, DynamicSoundEvent},
        attach_native_func,
        utils::intercept,
    };

    attach_native_func!(
        "Entity::QueueEvent",
        super::super::offsets::ENTITY_QUEUE_EVENT
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
                let dynamic: Ref<DynamicSoundEvent> = mem::transmute(event);
                if let Some(dynamic) = dynamic.fields() {
                    let entity = std::mem::transmute::<&IScriptable, &Entity>(&*i);
                    let entity_id = entity
                        .entity_id
                        .is_defined()
                        .then_some(entity.entity_id)
                        .or(None);
                    let emitter_name = entity.components.iter().find_map(|x| {
                        x.is_exactly_a::<AudioEmitterComponent>()
                            .then(|| x.clone().cast::<AudioEmitterComponent>().unwrap())
                            .and_then(|x| x.fields().map(|x| x.emitter_name))
                    });
                    passthru = !dynamic.enqueue_and_play(entity_id, emitter_name);
                    intercept!(
                        "Entity::QueueEvent for DynamicSoundEvent ({})",
                        dynamic.name.get()
                    );
                }
            } else if event.is_a::<DynamicEmitterEvent>() {
                let dynamic: Ref<DynamicEmitterEvent> = mem::transmute(event);
                if let Some(dynamic) = dynamic.fields() {
                    let entity = std::mem::transmute::<&IScriptable, &Entity>(&*i);
                    let entity_id = entity.entity_id;
                    passthru = !dynamic.enqueue_and_play(entity_id);
                    intercept!(
                        "Entity::QueueEvent for DynamicEmitterEvent ({}: {})",
                        dynamic.name.get(),
                        dynamic.tag_name.get(),
                    );
                }
            }

            frame.restore_args(state);
            if passthru {
                cb(i, frame as *mut _, a3, a4);
            }
        }
    }
}
