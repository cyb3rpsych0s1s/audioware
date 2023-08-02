use std::{fs::DirEntry, path::PathBuf};

use red4ext_rs::prelude::*;

use kira::{
    manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};

define_plugin! {
    name: "audioware",
    author: "Roms1383",
    version: 1:0:0,
    on_register: {
        register_function!("Initialize", initialize);
        register_function!("PlayAudio", play_audio);
    }
}

fn initialize() {
    let current = std::env::current_dir()?;
    let root = current.join("..").join("..").join("..");
    let redmod = root.join("mods");
    if redmod.exists() {
        if let Ok(mods) = std::fs::read_dir(redmod).and_then(|x| {
            Ok(x.into_iter()
                .filter_map(|x| x.ok())
                .filter(|x| x.metadata().and_then(|x| Ok(x.is_dir())).or_else(false))
                .collect::<Vec<DirEntry>>())
        }) {
            for dir in mods {
                if dir.path().join("customSounds").exists() && dir.path().join("info.json") {
                    // TODO
                }
            }
        }
    }
}

fn play_audio(name: CName) -> bool {
    if let Ok(manager) = AUDIO_MANAGER.lock() {
        // TODO
        let sound_data = StaticSoundData::from_file("sound.ogg", StaticSoundSettings::default())?;
        if let Ok() = manager.play(sound_data.clone()) {
            return true;
        }
    }
}

static AUDIO_MANAGER: Lazy<Mutex<AudioManager<CpalBackend>>> =
    Lazy::new(|| AudioManager::<CpalBackend>::new(AudioManagerSettings::default()));
