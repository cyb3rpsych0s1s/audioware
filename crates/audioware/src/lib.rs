#![doc(html_root_url = "https://cyb3rpsych0s1s.github.io/audioware")]
#![doc = include_str!("../../../README.md")]

include!(concat!(env!("OUT_DIR"), "/version.rs"));

use std::time::Instant;

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

use crate::abi::lifecycle::Lifecycle;

/// Audio [Plugin] for Cyberpunk 2077.
pub struct Audioware;

impl Plugin for Audioware {
    const NAME: &'static U16CStr = wcstr!("audioware");
    const AUTHOR: &'static U16CStr = wcstr!("Roms1383");
    const VERSION: SemVer = AUDIOWARE_VERSION;

    /// Initialize plugin.
    fn on_init(env: &SdkEnv) {
        abi::register_listeners(env);
        if let Err(e) = queue::spawn(env) {
            error!(env, "Error: {e}");
        }
    }

    /// Terminate plugin.
    fn on_exit(_: &SdkEnv) {
        queue::notify(Lifecycle::Terminate);
        let now = Instant::now();
        loop {
            if now.elapsed() > std::time::Duration::from_millis(50) {
                break;
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
