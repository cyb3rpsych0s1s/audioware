use std::{sync::RwLock, time::Instant};

use once_cell::sync::Lazy;
use red4ext_rs::{addr_hashes, hooks, log, types::IScriptable, PluginOps, SdkEnv};

use crate::engine::Engine;

hooks! {
   static HOOK: fn(i: *mut IScriptable) -> ();
}

static DELTA_TIME: Lazy<RwLock<Instant>> = Lazy::new(|| RwLock::new(Instant::now()));

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::ON_TRANSFORM_UPDATED);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(env, "attached hook for GameObject.OnTransformUpdated");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(i: *mut IScriptable, cb: unsafe extern "C" fn(i: *mut IScriptable)) {
    let now = Instant::now();
    let elapsed = DELTA_TIME
        .try_read()
        .as_deref()
        .map(|x| now.duration_since(*x).as_millis() > 1)
        .unwrap_or(false);
    if elapsed {
        // it's reasonable to miss a couple of updates
        if let Ok(x) = DELTA_TIME.try_write().as_deref_mut() {
            *x = now;
            Engine::sync_listener();
            Engine::sync_emitters();
        }
    }
    cb(i);
}
