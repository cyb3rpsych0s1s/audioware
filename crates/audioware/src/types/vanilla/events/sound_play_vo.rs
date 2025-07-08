use core::fmt;

use red4ext_rs::{
    NativeRepr, ScriptClass,
    class_kind::Native,
    types::{CName, EntityId, IScriptable},
};

use super::Event;

const PADDING_49: usize = 0x4A - 0x49;
const PADDING_4D: usize = 0x50 - 0x4D;
const PADDING_65: usize = 0x68 - 0x65;

#[repr(C)]
pub struct SoundPlayVO {
    base: Event,
    pub vo_context: CName,                                    // 40
    pub is_quest: bool,                                       // 48
    unk49: [u8; PADDING_49],                                  // 49
    pub ignore_frustum_check: bool,                           // 4A
    pub ignore_distance_check: bool,                          // 4B
    pub ignore_global_vo_limit_check: bool,                   // 4C
    unk4d: [u8; PADDING_4D],                                  // 4D
    pub debug_initial_context: CName,                         // 50
    pub answering_entity_id: EntityId,                        // 58
    pub overriding_voiceover_context: VoiceoverContext,       // 60
    pub overriding_voiceover_expression: VoiceoverExpression, // 61
    pub override_voiceover_expression: bool,                  // 62
    pub overriding_visual_style_value: u8,                    // 63
    pub override_visual_style: bool,                          // 64
    unk65: [u8; PADDING_65],                                  // 65
}

unsafe impl ScriptClass for SoundPlayVO {
    const NAME: &'static str = "entGameplayVOEvent";
    type Kind = Native;
}

impl AsRef<IScriptable> for SoundPlayVO {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(
    non_camel_case_types,
    clippy::enum_variant_names,
    reason = "see RED4ext.SDK"
)]
pub enum VoiceoverContext {
    Vo_Context_Quest = 0,
    Vo_Context_Community = 1,
    Vo_Context_Combat = 2,
    Vo_Context_Minor_Activity = 3,
    #[default]
    Default_Vo_Context = 5,
}

unsafe impl NativeRepr for VoiceoverContext {
    const NAME: &'static str = "VoiceoverContext";
}

impl fmt::Display for VoiceoverContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Vo_Context_Quest => "Quest",
                Self::Vo_Context_Community => "Community",
                Self::Vo_Context_Combat => "Combat",
                Self::Vo_Context_Minor_Activity => "Minor Activity",
                Self::Default_Vo_Context => "Context",
            }
        )
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(
    non_camel_case_types,
    clippy::enum_variant_names,
    reason = "see RED4ext.SDK"
)]
pub enum VoiceoverExpression {
    Vo_Expression_Spoken = 0,
    Vo_Expression_Phone = 1,
    Vo_Expression_InnerDialog = 2,
    Vo_Expression_Loudspeaker_Room = 3,
    Vo_Expression_Loudspeaker_Street = 4,
    Vo_Expression_Loudspeaker_City = 5,
    Vo_Expression_Radio = 6,
    Vo_Expression_GlobalTV = 7,
    Vo_Experession_Cb_Radio = 8,
    Vo_Expression_Cyberspace = 9,
    Vo_Expression_Possessed = 10,
    Vo_Expression_Helmet = 11,
}

unsafe impl NativeRepr for VoiceoverExpression {
    const NAME: &'static str = "VoiceoverExpression";
}

impl fmt::Display for VoiceoverExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Vo_Expression_Spoken => "Spoken",
                Self::Vo_Expression_Phone => "Phone",
                Self::Vo_Expression_InnerDialog => "InnerDialog",
                Self::Vo_Expression_Loudspeaker_Room => "Loudspeaker Room",
                Self::Vo_Expression_Loudspeaker_Street => "Loudspeaker Street",
                Self::Vo_Expression_Loudspeaker_City => "Loudspeaker City",
                Self::Vo_Expression_Radio => "Radio",
                Self::Vo_Expression_GlobalTV => "GlobalTV",
                Self::Vo_Experession_Cb_Radio => "Cb Radio",
                Self::Vo_Expression_Cyberspace => "Cyberspace",
                Self::Vo_Expression_Possessed => "Possessed",
                Self::Vo_Expression_Helmet => "Helmet",
            }
        )
    }
}
