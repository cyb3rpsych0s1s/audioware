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
        ScriptClass, VoidPtr,
        types::{IScriptable, Ref, StackFrame},
    };

    use crate::{Event, UIInGameNotificationRemoveEvent, attach_native_func, utils::intercept};

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
            frame.restore_args(state);

            let classname = event
                .fields()
                .map(|x| x.as_ref().class().name().as_str())
                .unwrap_or("unknown class");
            let dispatch = classname == UIInGameNotificationRemoveEvent::NAME;
            intercept!("inkMenuScenario::QueueEvent: {classname}",);
            cb(i, frame as *mut _, a3, a4);
            if dispatch {
                crate::engine::queue::notify(
                    crate::abi::lifecycle::Lifecycle::UIInGameNotificationRemove,
                );
            }
        }
    }
}
