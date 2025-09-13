use red4ext_rs::{
    NativeRepr,
    types::{CName, Cruid, IScriptable, ISerializable, Ref},
};

use crate::{VoiceoverContext, VoiceoverExpression};

const PADDING_45: usize = 0x48 - 0x45;

#[repr(C)]
pub struct SceneEvent {
    base: ISerializable,
    pub id: SceneEventId,               // 30
    pub type_: u32,                     // 38 (EventType is an empty enum in RED4ext.SDK)
    pub start_time: u32,                // 3C
    pub duration: u32,                  // 40
    pub execution_tag_flags: u8,        // 44
    unk45: [u8; PADDING_45],            // 45
    pub scaling_data: Ref<IScriptable>, // 48 (TODO: IScalingData)
}

unsafe impl NativeRepr for SceneEvent {
    const NAME: &'static str = "scnSceneEvent";
}

#[derive(Debug)]
#[repr(transparent)]
pub struct SceneEventId(f64);

unsafe impl NativeRepr for SceneEventId {
    const NAME: &'static str = "scnSceneEventId";
}
