use audioware_macros::FromMemory;
use red4ext_rs::{
    conv::ClassType,
    types::{CName, Ref},
};

use super::{event::Event, ISCRIPTABLE_SIZE};

const PAD_POST_SOUND_PLAY_EVENT: usize = 0x60 - 0x5D;
#[derive(Debug, FromMemory)]
pub struct SoundPlayEvent {
    iscriptable: [u8; ISCRIPTABLE_SIZE],
    pub sound_name: CName,                    // 40
    pub emitter_name: CName,                  // 48
    pub audio_tag: CName,                     // 50
    pub seek_time: f32,                       // 58
    pub play_unique: bool,                    // 5C
    uint8_t: [u8; PAD_POST_SOUND_PLAY_EVENT], // 5D
}

impl ClassType for SoundPlayEvent {
    type BaseClass = Event;
    const NAME: &'static str = "SoundPlayEvent";
    const NATIVE_NAME: &'static str = "gameaudioeventsPlaySound";
}

impl SoundPlayEvent {
    pub fn get_class_name(self: &Ref<Self>) -> CName {
        red4ext_rs::prelude::Ref::<Self>::upcast(self.clone()).get_class_name()
    }
    pub fn is_exactly_a(self: &Ref<Self>, class_name: CName) -> bool {
        red4ext_rs::prelude::Ref::<Self>::upcast(self.clone()).is_exactly_a(class_name)
    }
}
