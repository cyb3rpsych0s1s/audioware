#![doc(html_root_url = "https://cyb3rpsych0s1s.github.io/audioware")]
#![doc = include_str!("../../../README.md")]

include!(concat!(env!("OUT_DIR"), "/version.rs"));

use crossbeam::channel::{bounded, unbounded};
use red4ext_rs::{
    Exportable, Plugin, SdkEnv, SemVer, U16CStr, export_plugin_symbols, log::error, wcstr,
};
pub use types::*;

#[cfg(feature = "hot-reload")]
#[allow(dead_code, reason = "depends on feature enabled")]
pub mod debug;

mod abi;
mod cache;
mod config;
mod engine;
mod error;
mod hooks;
mod types;
mod utils;

use engine::queue;

use crate::{
    abi::{
        callback::Callback,
        command::Command,
        control::{DynamicEmitter, DynamicSound},
        lifecycle::Lifecycle,
    },
    engine::queue::{
        CALLBACKS, COMMAND, DYNAMIC_EMITTERS, DYNAMIC_SOUNDS, LIFECYCLE, THREAD, load,
    },
    hooks::detach,
    utils::{fails, lifecycle},
};

/// Audio [Plugin] for Cyberpunk 2077.
pub struct Audioware;

impl Plugin for Audioware {
    const NAME: &'static U16CStr = wcstr!("audioware");
    const AUTHOR: &'static U16CStr = wcstr!("Roms1383");
    const VERSION: SemVer = AUDIOWARE_VERSION;

    /// Initialize plugin.
    fn on_init(env: &SdkEnv) {
        lifecycle!("load plugin env");
        let Ok((engine, capacity)) = load(env) else {
            fails!("nooooooooooooooooooooo");
            return;
        };
        lifecycle!("initialize channels...");
        let (sl, rl) = bounded::<Lifecycle>(capacity * 4);
        let (sc, rc) = bounded::<Command>(capacity);
        let (se, re) = unbounded::<Callback>();
        let (sds, rds) = bounded::<DynamicSound>(capacity);
        let (sde, rde) = bounded::<DynamicEmitter>(capacity);
        let _ = LIFECYCLE.set(std::sync::RwLock::new(Some(sl)));
        let _ = COMMAND.set(std::sync::RwLock::new(Some(sc)));
        let _ = CALLBACKS.set(std::sync::RwLock::new(Some(se)));
        let _ = DYNAMIC_SOUNDS.set(std::sync::RwLock::new(Some(sds)));
        let _ = DYNAMIC_EMITTERS.set(std::sync::RwLock::new(Some(sde)));
        lifecycle!("initialized channels");
        abi::register_listeners(env);
        if let Err(e) = queue::spawn(env, rl, rc, re, rds, rde, engine) {
            error!(env, "Error: {e}");
        }
    }

    /// Terminate plugin.
    fn on_exit(env: &SdkEnv) {
        queue::notify(Lifecycle::Terminate);
        detach(env);
        if let Some(thread) = THREAD.get() {
            loop {
                if let Ok(mut guard) = thread.try_lock() {
                    if let Some(thread) = guard.take() {
                        while !thread.is_finished() {
                            continue;
                        }
                        if let Err(e) = thread.join() {
                            fails!("unable to join thread: {e:?}");
                        }
                    }
                    break;
                }
            }
        }
    }

    /// Register types in [RTTI][red4ext_rs::RttiSystem].
    #[allow(clippy::transmute_ptr_to_ref, reason = "upstream lint")]
    fn exports() -> impl Exportable {
        abi::exports()
    }
}

impl Audioware {
    fn once_rtti_registered() {
        use red4ext_rs::PluginOps;
        hooks::attach(Audioware::env());
    }
    fn once_exit_initialization() {
        queue::notify(abi::lifecycle::Lifecycle::ReportInitialization);
    }
}

export_plugin_symbols!(Audioware);
