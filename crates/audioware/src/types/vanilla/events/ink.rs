use red4ext_rs::{
    ScriptClass,
    class_kind::Native,
    types::{Cruid, RedString},
};

use crate::Event;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct VORequestEvt {
    base: Event,
    pub vo_id: Cruid,
    pub speaker_name: RedString,
}

unsafe impl ScriptClass for VORequestEvt {
    type Kind = Native;
    const NAME: &'static str = "inkVORequestEvt";
}

impl AsRef<Event> for VORequestEvt {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}
