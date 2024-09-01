use std::{sync::RwLock, time::Instant};

use once_cell::sync::Lazy;
use red4ext_rs::{addr_hashes, hooks, types::IScriptable, SdkEnv};

use crate::engine::Engine;

hooks! {
   static HOOK: fn(i: *mut IScriptable) -> ();
}

static SYNC_DELTA_TIME: Lazy<RwLock<Instant>> = Lazy::new(|| RwLock::new(Instant::now()));
static RECLAIM_DELTA_TIME: Lazy<RwLock<Instant>> = Lazy::new(|| RwLock::new(Instant::now()));

pub const SYNC_ELAPSED_MILLIS: u128 = 20;
pub const RECLAIM_ELAPSED_MILLIS: u128 = 60 * 1000;

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::ON_TRANSFORM_UPDATED);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::lifecycle!("attached hook for GameObject.OnTransformUpdated");
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(i: *mut IScriptable, cb: unsafe extern "C" fn(i: *mut IScriptable)) {
    cb(i);
    if !Engine::should_sync_emitters() {
        return;
    }
    let now = Instant::now();
    let (sync, reclaim) = (
        SYNC_DELTA_TIME
            .try_read()
            .as_deref()
            .map(|x| now.duration_since(*x).as_millis() > SYNC_ELAPSED_MILLIS)
            .unwrap_or(false),
        RECLAIM_DELTA_TIME
            .try_read()
            .as_deref()
            .map(|x| now.duration_since(*x).as_millis() > RECLAIM_ELAPSED_MILLIS)
            .unwrap_or(false),
    );
    if sync {
        if let Ok(x) = SYNC_DELTA_TIME.try_write().as_deref_mut() {
            *x = now;
            Engine::sync_listener();
            Engine::sync_emitters();
        }
    }
    if reclaim {
        if let Ok(x) = RECLAIM_DELTA_TIME.try_write().as_deref_mut() {
            *x = now;
            Engine::reclaim();
        }
    }
}
