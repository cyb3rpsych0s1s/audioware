use red4ext_rs::SdkEnv;

pub fn attach_hooks(env: &SdkEnv) {
    area_entered::attach_hook(env);
    area_exited::attach_hook(env);
}

mod area_entered {
    use crate::{attach_native_event, AreaEnteredEvent, TriggerEvent};
    use red4ext_rs::types::IScriptable;
    use red4ext_rs::ScriptClass;

    attach_native_event!(
        super::super::super::offsets::AREA_ENTERED_EVENT,
        crate::AreaEnteredEvent
    );

    unsafe extern "C" fn detour(
        a1: *mut IScriptable,
        a2: *mut AreaEnteredEvent,
        cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut AreaEnteredEvent),
    ) {
        let event = &*a2;
        let TriggerEvent {
            trigger_id,
            component_name,
            ..
        } = event.as_ref();
        crate::utils::lifecycle!(
            "intercepted {}:
    - trigger_id: {trigger_id}
    - component_name: {component_name}",
            AreaEnteredEvent::NAME
        );
        cb(a1, a2);
    }
}

mod area_exited {
    use crate::{attach_native_event, AreaExitedEvent, TriggerEvent};
    use red4ext_rs::types::IScriptable;
    use red4ext_rs::ScriptClass;

    attach_native_event!(
        super::super::super::offsets::AREA_EXITED_EVENT,
        crate::AreaExitedEvent
    );

    unsafe extern "C" fn detour(
        a1: *mut IScriptable,
        a2: *mut AreaExitedEvent,
        cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut AreaExitedEvent),
    ) {
        let event = &*a2;
        let TriggerEvent {
            trigger_id,
            component_name,
            ..
        } = event.as_ref();
        crate::utils::lifecycle!(
            "intercepted {}:
    - trigger_id: {trigger_id}
    - component_name: {component_name}",
            AreaExitedEvent::NAME
        );
        cb(a1, a2);
    }
}
