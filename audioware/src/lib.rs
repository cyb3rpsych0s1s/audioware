use red4ext_rs::plugin::Version;
use red4ext_rs::register_function;
use red4ext_rs::{define_trait_plugin, plugin::Plugin};

mod engine;
pub mod natives;

struct Audioware;

impl Plugin for Audioware {
    const VERSION: Version = Version::new(0, 0, 1);

    fn register() {
        engine::setup();
        register_function!(
            "Audioware.UpdateEngineState",
            crate::natives::update_engine_state
        );
    }

    fn post_register() {}

    fn unload() {}
}

define_trait_plugin! (
    name: "audioware",
    author: "Roms1383",
    plugin: Audioware
);
