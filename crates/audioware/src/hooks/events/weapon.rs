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
    use crate::{attach_native_event, Event, StopFiringEvent, StopWeaponFire};
    use red4ext_rs::types::CName;
    use red4ext_rs::types::IScriptable;
    use red4ext_rs::ScriptClass;

    attach_native_event!(
        super::super::super::offsets::WEAPON_STOP_FIRING_EVENT,
        crate::Event
    );

    unsafe extern "C" fn detour(
        a1: *mut IScriptable,
        a2: *mut Event,
        cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut Event),
    ) {
        let event = &*a2;
        if event
            .as_ref()
            .class()
            .name()
            .eq(&CName::new(StopFiringEvent::NAME))
        {
            crate::utils::lifecycle!("intercepted {}", StopFiringEvent::NAME);
        } else if event
            .as_ref()
            .class()
            .name()
            .eq(&CName::new(StopWeaponFire::NAME))
        {
            crate::utils::lifecycle!("intercepted {}", StopWeaponFire::NAME);
        }
        cb(a1, a2);
    }
}
