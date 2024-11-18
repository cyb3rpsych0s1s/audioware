// use std::mem;

// use red4ext_rs::{
//     types::{CName, IScriptable},
//     ScriptClass,
// };

// use crate::{hooks::NativeHandler, DialogLine, DialogLineEventData, Entity};

// pub struct Handler;

// impl NativeHandler<{ super::super::offsets::EVENT_DIALOGLINE }> for Handler {
//     type EventClass = DialogLine;
//     fn detour<'a>(
//         this: &IScriptable,
//         event: &'a mut Self::EventClass,
//     ) -> Option<&'a mut Self::EventClass> {
//         let DialogLine { data, .. } = event;
//         let DialogLineEventData {
//             string_id,
//             context,
//             expression,
//             is_player,
//             is_rewind,
//             is_holocall,
//             custom_vo_event,
//             seek_time,
//             playback_speed_parameter,
//             ..
//         } = data;
//         let id = this
//             .as_ref()
//             .class()
//             .name()
//             .eq(&CName::new(Entity::NAME))
//             .then_some(unsafe { mem::transmute::<&IScriptable, &Entity>(this) })
//             .map(|x| x.entity_id);
//         crate::utils::lifecycle!(
//             "intercepted DialogLine on {id:?}:
// - data.string_id {string_id:?}
// - data.context {context}
// - data.expression {expression}
// - data.is_player {is_player}
// - data.is_rewind {is_rewind}
// - data.is_holocall {is_holocall}
// - data.custom_vo_event {custom_vo_event}
// - data.seek_time {seek_time}
// - data.playback_speed_parameter {playback_speed_parameter}",
//         );
//         Some(event)
//     }
// }
