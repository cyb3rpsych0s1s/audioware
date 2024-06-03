use audioware_macros::FromMemory;
use red4ext_rs::types::Ref;
use red4ext_rs::{conv::ClassType, types::CName};

use super::iscriptable::{IScriptable, ISCRIPTABLE_SIZE};

#[derive(Debug, Clone, FromMemory)]
#[repr(C)]
pub struct Event {
    pub iscriptable: [u8; ISCRIPTABLE_SIZE],
}

impl ClassType for Event {
    type BaseClass = IScriptable;
    const NAME: &'static str = "Event";
    const NATIVE_NAME: &'static str = "redEvent";
}

impl Event {
    pub fn sound_name(self: &Ref<Self>) -> CName {
        CName::new("None")
    }
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", Self::NAME, Self::NATIVE_NAME)
    }
}
