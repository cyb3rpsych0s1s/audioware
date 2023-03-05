use std::{os::unix::fs::DirEntryExt2, path::PathBuf};

use red4ext_rs::prelude::*;

mod fmod;

struct Audioware;

define_plugin! {
    name: "audioware",
    author: "Roms1383",
    version: 1:0:0,
    plugin: Audioware
}

fn load_bank(name: String) {
    let studio = fmod::load();
    if let Some(folder) = get_mod_custom_sounds_path(name.as_str()) {
        studio.load_bank_file(
            folder.join(name.as_str()).with_extension("bank"),
            FMOD_STUDIO_LOAD_BANK_NORMAL,
        )?;
        studio.load_bank_file(
            folder.join(name.as_str()).with_extension("strings.bank"),
            FMOD_STUDIO_LOAD_BANK_NORMAL,
        )?;
    }
}

fn get_mod_custom_sounds_path(folder: &str) -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let folder: &Path = folder.as_ref();
    let path = exe
        .parent()?
        .parent()?
        .parent()?
        .join("mods")
        .join(folder)
        .join("customSounds");
    Some(path)
}

impl Plugin for Audioware {
    fn post_register() {
        register_function!("Audioware.LoadBank", load_bank);
    }

    fn unload() {
        fmod::unload();
    }
}
