#![doc(html_root_url = "https://cyb3rpsych0s1s.github.io/audioware")]
#![doc = include_str!("../../../README.md")]

include!(concat!(env!("OUT_DIR"), "/version.rs"));

#[cfg(target_os = "macos")]
use red4ext_rs::log::info;
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

use crate::{abi::lifecycle::Lifecycle, engine::queue::THREAD, hooks::detach, utils::fails};

/// Audio [Plugin] for Cyberpunk 2077.
pub struct Audioware;

impl Plugin for Audioware {
    const NAME: &'static U16CStr = wcstr!("audioware");
    const AUTHOR: &'static U16CStr = wcstr!("Roms1383");
    const VERSION: SemVer = AUDIOWARE_VERSION;

    /// Initialize plugin.
    fn on_load(env: &SdkEnv) {
        // macOS, private loader: `Main` runs from the loader's dylib constructor, before the game's static initializers. The RTTI system, the
        // name pool and the memory pools do not exist yet (red4ext-rs `internal::is_private_loader`), and the audio engine builds names from
        // the manifests, so the engine, the RTTI callbacks and the hooks start later, at `audioware_start`. The game-state listener
        // (`Initialization` exit) is not used there either: the loader attaches none of its game application hooks, so no state ever runs.
        #[cfg(target_os = "macos")]
        if red4ext_rs::internal::is_private_loader() {
            info!(
                env,
                "Audioware: private loader, the audio engine and the hooks start at audioware_start (hook mode {})",
                hooks::macos::mode().as_str()
            );
            return;
        }
        abi::register_listeners(env);
        if let Err(e) = queue::spawn() {
            error!(env, "Error: {e}");
        }
    }

    /// Terminate plugin.
    fn on_unload(env: &SdkEnv) {
        detach(env);
        queue::notify(Lifecycle::Terminate);
        if let Some(thread) = THREAD.get() {
            loop {
                if let Ok(mut guard) = thread.try_lock() {
                    if let Some(thread) = guard.take()
                        && let Err(e) = thread.join()
                    {
                        fails!("unable to join thread: {e:?}");
                    }
                    break;
                } else {
                    // Contended (rare) — yield the time slice instead of spinning.
                    std::thread::yield_now();
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

// macOS: `audioware_registerNatives` is the entry point of the private loader's launcher (`frida-modded.js`, NATREG list): it registers the
// natives, once, at InitScripts. A build against the upstream red4ext-rs, whose macro takes one argument, uses the one argument form elsewhere.
#[cfg(target_os = "macos")]
export_plugin_symbols!(Audioware, audioware_registerNatives);
#[cfg(not(target_os = "macos"))]
export_plugin_symbols!(Audioware);

/// macOS, private loader: starts the audio engine and attaches the hooks, once.
///
/// `frida-modded.js` calls it right after `audioware_registerNatives`, both at InitScripts entry: the RTTI system and the name pool exist
/// then (the launcher checks the ready flag first), and the natives the scripts call are registered. On Windows the same work happens in
/// `Plugin::on_load` (the engine) and in the RTTI post-register callback (the hooks), which the Mac game never runs.
/// Does nothing under another loader: there `on_load` and the post-register callback do it.
#[cfg(target_os = "macos")]
#[unsafe(no_mangle)]
pub extern "C" fn audioware_start() {
    use red4ext_rs::PluginOps;
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        if !red4ext_rs::internal::is_private_loader() {
            return;
        }
        let env = Audioware::env();
        info!(env, "Audioware: starting the audio engine and the hooks");
        // The game-state listener never fires here, so the engine thread reports the initialization itself once it runs.
        queue::REPORT_ON_START.store(true, std::sync::atomic::Ordering::Release);
        if let Err(e) = queue::spawn() {
            error!(env, "Error: {e}");
        }
        Audioware::once_rtti_registered();
    });
}
