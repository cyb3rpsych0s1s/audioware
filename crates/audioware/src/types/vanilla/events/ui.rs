use red4ext_rs::{ScriptClass, class_kind::Scripted};

use crate::Event;

pub struct UIInGameNotificationRemoveEvent {
    base: Event,
}

unsafe impl ScriptClass for UIInGameNotificationRemoveEvent {
    const NAME: &'static str = "UIInGameNotificationRemoveEvent";
    type Kind = Scripted;
}
