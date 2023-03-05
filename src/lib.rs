use red4ext_rs::prelude::*;

mod fmod;

struct Audioware;

define_trait_plugin! {
    name: "audioware",
    author: "Roms1383",
    plugin: Audioware
}

fn load_bank(name: String) {
    fmod::load(name);
}

impl Plugin for Audioware {
    fn post_register() {
        register_function!("Audioware.LoadBank", load_bank);
    }

    fn unload() {
        fmod::unload();
    }

    fn version() -> Version {
        Version { major: 1, minor: 0, patch: 0 }
    }
}
