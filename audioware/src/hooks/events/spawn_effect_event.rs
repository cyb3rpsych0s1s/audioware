use red4ext_rs::{addr_hashes, hooks, log, types::IScriptable, PluginOps, SdkEnv};

use crate::{types::SpawnEffectEvent, Audioware};

hooks! {
   static HOOK: fn(a1: *mut IScriptable, a2: *mut SpawnEffectEvent) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(crate::hooks::offsets::SPAWN_EFFECT_EVENT_HANDLER);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(env, "attached hook for SpawnEffectEvent event handler");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    a1: *mut IScriptable,
    a2: *mut SpawnEffectEvent,
    cb: unsafe extern "C" fn(a1: *mut IScriptable, a2: *mut SpawnEffectEvent),
) {
    if !a2.is_null() {
        let &SpawnEffectEvent {
            e3hack_defer_count,
            effect_name,
            id_for_randomized_effect,
            effect_instance_name,
            persist_on_detach,
            break_all_loops,
            break_all_on_destroy,
            ..
        } = unsafe { &*a2 };
        log::info!(
            Audioware::env(),
            "intercepted SpawnEffectEvent:
- e3hack_defer_count: {e3hack_defer_count}
- effect_name: {effect_name}
- id_for_randomized_effect: {}
- effect_instance_name: {effect_instance_name}
- persist_on_detach: {persist_on_detach}
- break_all_loops: {break_all_loops}
- break_all_on_destroy: {break_all_on_destroy}",
            i64::from(id_for_randomized_effect)
        );
    } else {
        log::info!(Audioware::env(), "intercepted SpawnEffectEvent (null)");
    }

    cb(a1, a2);
}
