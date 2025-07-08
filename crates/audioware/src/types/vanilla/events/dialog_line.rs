use red4ext_rs::{
    NativeRepr, ScriptClass,
    class_kind::Native,
    types::{CName, Cruid, IScriptable},
};

use super::{Event, VoiceoverContext, VoiceoverExpression};

#[repr(C)]
pub struct DialogLine {
    base: Event,
    pub data: DialogLineEventData,
}

unsafe impl ScriptClass for DialogLine {
    const NAME: &'static str = "gameaudioeventsDialogLine";
    type Kind = Native;
}

impl AsRef<Event> for DialogLine {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

#[repr(C)]
pub struct DialogLineEnd {
    base: Event,
}

unsafe impl ScriptClass for DialogLineEnd {
    const NAME: &'static str = "gameaudioeventsDialogLineEnd";
    type Kind = Native;
}

const PADDING_4C: usize = 0x50 - 0x4C;

#[repr(C)]
pub struct StopDialogLine {
    base: Event,
    pub string_id: Cruid,    // 40
    pub fade_out: f32,       // 48
    unk4c: [u8; PADDING_4C], // 4C
}

unsafe impl ScriptClass for StopDialogLine {
    const NAME: &'static str = "gameaudioeventsStopDialogLine";
    type Kind = Native;
}

impl AsRef<Event> for StopDialogLine {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for StopDialogLine {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

const PADDING_A: usize = 0x10 - 0xA;
const PADDING_13: usize = 0x18 - 0x13;

#[repr(C)]
pub struct DialogLineEventData {
    pub string_id: Cruid,                // 00
    pub context: VoiceoverContext,       // 08
    pub expression: VoiceoverExpression, // 09
    unk0a: [u8; PADDING_A],              // A
    pub is_player: bool,                 // 10
    pub is_rewind: bool,                 // 11
    pub is_holocall: bool,               // 12
    unk13: [u8; PADDING_13],             // 13
    pub custom_vo_event: CName,          // 18
    pub seek_time: f32,                  // 20
    pub playback_speed_parameter: f32,   // 24
}

unsafe impl NativeRepr for DialogLine {
    const NAME: &'static str = "audioDialogLineEventData";
}
