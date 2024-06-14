mod add_trigger_effect;
mod global_parameter;
mod parameter;
mod play;
mod play_on_emitter;
mod state;
mod stop;
mod switch;

pub use add_trigger_effect::HookAudioSystemAddTriggerEffect;
pub use global_parameter::HookAudioSystemGlobalParameter;
pub use parameter::HookAudioSystemParameter;
pub use play::HookAudioSystemPlay;
pub use play_on_emitter::HookAudioSystemPlayOnEmitter;
pub use state::HookAudioSystemState;
pub use stop::HookAudioSystemStop;
pub use switch::HookAudioSystemSwitch;

use red4ext_rs::types::{CName, EntityId};

use audioware_bank::Banks;

pub fn audioware_exists((sound_name, ..): &(CName, EntityId, CName)) -> bool {
    Banks::exists(sound_name)
}
