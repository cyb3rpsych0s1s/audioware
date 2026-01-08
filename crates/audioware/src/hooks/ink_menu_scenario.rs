use red4ext_rs::SdkEnv;

pub fn attach_hooks(env: &SdkEnv) {
    switch_to_scenario::attach_hook(env);
    queue_event::attach_hook(env);
}

mod switch_to_scenario {
    use red4ext_rs::{
        VoidPtr,
        types::{CName, IScriptable, Ref, StackFrame},
    };

    use crate::{abi::lifecycle::Lifecycle, attach_native_func, engine::queue, utils::intercept};

    attach_native_func!(
        "inkMenuScenario::SwitchToScenario",
        super::super::offsets::INKMENUSCENARIO_SWITCH_TO_SCENARIO
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

            let name: CName = StackFrame::get_arg(frame);
            let _user_data: Ref<IScriptable> = StackFrame::get_arg(frame);
            frame.restore_args(state);

            intercept!("inkMenuScenario::SwitchToScenario: {}", name.as_str());
            cb(i, frame as *mut _, a3, a4);

            queue::notify(Lifecycle::SwitchToScenario(name));
        }
    }
}

mod queue_event {
    use red4ext_rs::{
        VoidPtr,
        types::{IScriptable, Ref, StackFrame},
    };

    use crate::{
        Event, UIInGameNotificationRemoveEvent, abi::DynamicSoundEvent, attach_native_func,
        utils::intercept,
    };

    attach_native_func!(
        "inkMenuScenario::QueueEvent",
        super::super::offsets::INKMENUSCENARIO_QUEUE_EVENT
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
            let mut late_dispatch = false;
            if event.is_a::<DynamicSoundEvent>() {
                let dynamic: Ref<DynamicSoundEvent> = std::mem::transmute(event);
                if let Some(dynamic) = dynamic.fields() {
                    passthru = !dynamic.enqueue_and_play(None, None);
                    intercept!(
                        "inkMenuScenario::QueueEvent for DynamicSoundEvent ({})",
                        dynamic.name.get()
                    );
                }
            } else if event.is_a::<UIInGameNotificationRemoveEvent>() {
                late_dispatch = true;
            }

            frame.restore_args(state);
            if passthru {
                cb(i, frame as *mut _, a3, a4);
            }
            if late_dispatch {
                crate::engine::queue::notify(
                    crate::abi::lifecycle::Lifecycle::UIInGameNotificationRemove,
                );
            }
        }
    }
}
