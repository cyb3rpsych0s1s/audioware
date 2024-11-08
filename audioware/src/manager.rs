use std::{ops::DerefMut, sync::OnceLock};

use kira::manager::AudioManager;
use parking_lot::RwLock;

static MANAGER: OnceLock<RwLock<AudioManager>> = OnceLock::new();
// static PLUGIN_HANDLES: AtomicU8 = AtomicU8::new(PluginState::Uninitialized as u8);

#[derive(Clone)]
pub struct Manager;
impl Manager {
    pub fn stop(&self) {
        let num_clocks = MANAGER.get().unwrap().read().num_clocks();
    }
}
