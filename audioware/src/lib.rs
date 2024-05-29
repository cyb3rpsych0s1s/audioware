#![feature(arbitrary_self_types)]

use bank::Banks;
use red4ext_rs::{
    define_trait_plugin,
    plugin::{Plugin, Version},
};

mod bank;
mod engine;
pub mod manifest;
pub mod utils;

struct Audioware;

impl Plugin for Audioware {
    const VERSION: Version = Version::new(0, 8, 11);

    fn register() {
        red4ext_rs::info!("on register audioware");
        match Banks::setup() {
            Ok(report) => {
                red4ext_rs::info!("successfully initialized:\n{report}");
            }
            Err(e) => {
                red4ext_rs::error!("{e}");
            }
        };
    }

    fn post_register() {
        red4ext_rs::info!("on post register audioware");
    }

    fn unload() {
        red4ext_rs::info!("on unload audioware");
    }

    fn is_version_independent() -> bool {
        false
    }
}

define_trait_plugin! (
    name: "audioware",
    author: "Roms1383",
    plugin: Audioware
);
