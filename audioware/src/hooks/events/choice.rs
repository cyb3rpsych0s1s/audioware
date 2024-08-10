use red4ext_rs::{addr_hashes, hooks, log, types::IScriptable, PluginOps, SdkEnv};

use crate::{types::ChoiceEvent, Audioware};

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut ChoiceEvent) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(crate::hooks::offsets::INTERACTION_CHOICE_EVENT_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(env, "attached hook for ChoiceEvent event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut ChoiceEvent,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut ChoiceEvent),
) {
    if !a2.is_null() {
        let &ChoiceEvent {
            ref choice,
            action_type,
            ..
        } = unsafe { &*a2 };
        log::info!(
            Audioware::env(),
            "intercepted ChoiceEvent:
- choice: {choice:#?}
- action_type: {action_type}",
        );
    } else {
        log::info!(Audioware::env(), "intercepted ChoiceEvent (null)");
    }

    cb(a1, a2);
}
