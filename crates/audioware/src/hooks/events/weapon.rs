pub fn attach_hook(env: &red4ext_rs::SdkEnv) {
    pre_fire::attach_hook(env);
    stop_firing::attach_hook(env);
}

mod pre_fire {
    use crate::{attach_native_event, PreFireEvent};
    use red4ext_rs::types::IScriptable;
    use red4ext_rs::ScriptClass;

    attach_native_event!(
        super::super::super::offsets::WEAPON_PRE_FIRE_EVENT,
        crate::PreFireEvent
    );

    unsafe extern "C" fn detour(
        a1: *mut IScriptable,
        a2: *mut PreFireEvent,
        cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut PreFireEvent),
    ) {
        let event = &*a2;
        let PreFireEvent { .. } = event;
        crate::utils::lifecycle!("intercepted {}", PreFireEvent::NAME);
        cb(a1, a2);
    }
}

mod stop_firing {
    use crate::{attach_native_event, StopFiringEvent};
    use red4ext_rs::types::IScriptable;
    use red4ext_rs::ScriptClass;

    attach_native_event!(
        super::super::super::offsets::WEAPON_STOP_FIRING_EVENT,
        crate::StopFiringEvent
    );

    unsafe extern "C" fn detour(
        a1: *mut IScriptable,
        a2: *mut StopFiringEvent,
        cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut StopFiringEvent),
    ) {
        let event = &*a2;
        let StopFiringEvent { .. } = event;
        crate::utils::lifecycle!("intercepted {}", StopFiringEvent::NAME);
        cb(a1, a2);
    }
}
