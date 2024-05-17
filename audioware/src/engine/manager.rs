use std::sync::Mutex;

use kira::manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings};
use once_cell::sync::OnceCell;

pub fn audio_manager() -> &'static Mutex<AudioManager<DefaultBackend>> {
    static INSTANCE: OnceCell<Mutex<AudioManager<DefaultBackend>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        Mutex::new(
            AudioManager::new(AudioManagerSettings::default()).expect("instantiate audio manager"),
        )
    })
}
