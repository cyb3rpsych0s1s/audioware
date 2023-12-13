use audioware_types::interop::iscriptable::IScriptable;
use red4ext_rs::types::Ref;
use red4ext_rs::{conv::ClassType, types::CName};

use super::sound_play_event::SoundPlayEvent;

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

impl Event {
    pub fn sound_name(self: &Ref<Self>) -> CName {
        if self.is_exactly_a(CName::new(SoundPlayEvent::NATIVE_NAME)) {
            return unsafe { std::mem::transmute::<&Ref<Event>, &Ref<SoundPlayEvent>>(self) }
                .sound_name
                .clone();
        }
        CName::new("None")
    }
}
