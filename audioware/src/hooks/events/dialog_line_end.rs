use std::mem;

use red4ext_rs::{
    types::{CName, IScriptable},
    ScriptClass,
};

use crate::{hooks::NativeHandler, Entity, Event};

#[allow(non_camel_case_types)]
pub struct Handler;

impl NativeHandler<{ super::super::offsets::EVENT_DIALOGLINEEND }> for Handler {
    type EventClass = Event;
    fn detour<'a>(this: &IScriptable, event: &'a mut crate::Event) -> Option<&'a mut crate::Event> {
        let id = this
            .as_ref()
            .class()
            .name()
            .eq(&CName::new(Entity::NAME))
            .then_some(unsafe { mem::transmute::<&IScriptable, &Entity>(this) })
            .map(|x| x.entity_id);
        crate::utils::lifecycle!("intercepted DialogLineEnd on {id:?}",);
        Some(event)
    }
}
