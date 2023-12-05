use red4ext_rs::types::Ref;
use red4ext_rs::{conv::ClassType, macros::redscript_import, types::CName};

use crate::FromMemory;

#[repr(transparent)]
pub struct IScriptable(red4ext_rs::types::IScriptable);

impl ClassType for IScriptable {
    type BaseClass = red4ext_rs::types::IScriptable;

    const NAME: &'static str = red4ext_rs::types::IScriptable::NAME;
    const NATIVE_NAME: &'static str = red4ext_rs::types::IScriptable::NATIVE_NAME;
}

#[redscript_import]
impl IScriptable {
    /// public native func GetClassName() -> CName
    #[redscript(native)]
    pub fn get_class_name(self: &Ref<Self>) -> CName;
    /// public native func IsExactlyA(className: CName) -> Bool
    #[redscript(native)]
    pub fn is_exactly_a(self: &Ref<Self>, class_name: CName) -> bool;
}

#[derive(Debug)]
pub struct Event;

impl ClassType for Event {
    type BaseClass = IScriptable;
    const NAME: &'static str = "redEvent";
}

impl Event {
    pub fn get_class_name(self: &Ref<Self>) -> CName {
        red4ext_rs::prelude::Ref::<Self>::upcast(self.clone()).get_class_name()
    }
    pub fn is_exactly_a(self: &Ref<Self>, class_name: CName) -> bool {
        red4ext_rs::prelude::Ref::<Self>::upcast(self.clone()).is_exactly_a(class_name)
    }
}

const PAD_ISCRIPTABLE: usize = 0x40;
const PAD_POST_SOUND_PLAY_EVENT: usize = 0x60 - 0x5D;
#[derive(Debug)]
pub struct SoundPlayEvent {
    iscriptable: [u8; PAD_ISCRIPTABLE],
    pub sound_name: CName,                        // 40
    pub emitter_name: CName,                      // 48
    pub audio_tag: CName,                         // 50
    pub seek_time: f32,                           // 58
    pub play_unique: bool,                        // 5C
    pub uint8_t: [u8; PAD_POST_SOUND_PLAY_EVENT], // 5D
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

unsafe impl FromMemory for SoundPlayEvent {
    fn from_memory(address: usize) -> Self {
        let iscriptable: [u8; PAD_ISCRIPTABLE] = unsafe {
            *core::slice::from_raw_parts::<[u8; PAD_ISCRIPTABLE]>(
                address as *const [u8; PAD_ISCRIPTABLE],
                1,
            )
            .get_unchecked(0)
        };
        let sound_name: CName = unsafe {
            core::slice::from_raw_parts::<CName>((address + 0x40) as *const CName, 1)
                .get_unchecked(0)
                .clone()
        };
        let emitter_name: CName = unsafe {
            core::slice::from_raw_parts::<CName>((address + 0x48) as *const CName, 1)
                .get_unchecked(0)
                .clone()
        };
        let audio_tag: CName = unsafe {
            core::slice::from_raw_parts::<CName>((address + 0x50) as *const CName, 1)
                .get_unchecked(0)
                .clone()
        };
        let seek_time: f32 = unsafe {
            *core::slice::from_raw_parts::<f32>((address + 0x58) as *const f32, 1).get_unchecked(0)
        };
        let play_unique: bool = unsafe {
            *core::slice::from_raw_parts::<bool>((address + 0x5C) as *const bool, 1)
                .get_unchecked(0)
        };
        let uint8_t: [u8; PAD_POST_SOUND_PLAY_EVENT] = unsafe {
            *core::slice::from_raw_parts::<[u8; PAD_POST_SOUND_PLAY_EVENT]>(
                (address + 0x5D) as *const [u8; PAD_POST_SOUND_PLAY_EVENT],
                1,
            )
            .get_unchecked(0)
        };
        Self {
            iscriptable,
            sound_name,
            emitter_name,
            audio_tag,
            seek_time,
            play_unique,
            uint8_t,
        }
    }
}
