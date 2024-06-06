mod global_parameter;
mod parameter;
mod play;
mod play_on_emitter;
mod stop;

pub use global_parameter::HookAudioSystemGlobalParameter;
pub use parameter::HookAudioSystemParameter;
pub use play::HookAudioSystemPlay;
pub use play_on_emitter::HookAudioSystemPlayOnEmitter;
pub use stop::HookAudioSystemStop;

use red4ext_rs::types::{CName, EntityId};

use crate::bank::Banks;

pub fn audioware_exists(params: &(CName, EntityId, CName)) -> bool {
    let (sound_name, ..) = params;
    Banks::exists(sound_name)
}
