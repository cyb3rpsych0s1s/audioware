use red4ext_rs::plugin::Version;
use red4ext_rs::{define_trait_plugin, plugin::Plugin};

mod engine;

struct Audioware;

impl Plugin for Audioware {
    const VERSION: Version = Version::new(0, 0, 1);

    fn register() {
        engine::setup();
    }

    fn post_register() {}

    fn unload() {}
}

define_trait_plugin! (
    name: "audioware",
    author: "Roms1383",
    plugin: Audioware
);
