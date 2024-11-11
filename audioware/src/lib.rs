#![doc(html_root_url = "https://cyb3rpsych0s1s.github.io/audioware")]
#![doc = include_str!("../../README.md")]

include!(concat!(env!("OUT_DIR"), "/version.rs"));

use red4ext_rs::{
    export_plugin_symbols, log::error, wcstr, Exportable, Plugin, SdkEnv, SemVer, U16CStr,
};
pub use types::*;

mod abi;
mod config;
mod engine;
mod error;
mod queue;
mod states;
mod types;
mod utils;

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

    /// Register types in [RTTI][RttiSystem].
    #[allow(clippy::transmute_ptr_to_ref)] // upstream lint
    fn exports() -> impl Exportable {
        abi::exports()
    }
}

export_plugin_symbols!(Audioware);
