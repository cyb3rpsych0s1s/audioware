use core::fmt;

use red4ext_rs::{
    class_kind::Native,
    types::{CName, IScriptable, RedArray, RedString, TweakDbId, Variant, WeakRef},
    NativeRepr, ScriptClass,
};

use crate::types::{GameObject, Vector3};

use super::Event;

#[repr(C)]
pub struct InteractionBaseEvent {
    base: Event,
    pub hotspot: WeakRef<GameObject>,   // 40
    pub activator: WeakRef<GameObject>, // 50
    pub layer_data: LayerData,          // 60
}

unsafe impl ScriptClass for InteractionBaseEvent {
    type Kind = Native;
    const NAME: &'static str = "gameinteractionsInteractionBaseEvent";
}

impl fmt::Debug for InteractionBaseEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InteractionBaseEvent")
            .field("base", &self.base)
            .field("layer_data", &self.layer_data)
            .finish()
    }
}

impl AsRef<Event> for InteractionBaseEvent {
    fn as_ref(&self) -> &Event {
        &self.base
    }
}

impl AsRef<IScriptable> for InteractionBaseEvent {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct LayerData {
    pub tag: CName,
}

unsafe impl NativeRepr for LayerData {
    const NAME: &'static str = "gameinteractionsLayerData";
}

const PADDING_10C: usize = 0x110 - 0x10C;

#[derive(Debug)]
#[repr(C)]
pub struct ChoiceEvent {
    base: InteractionBaseEvent,
    pub choice: InteractionChoice, // 68
    pub action_type: ActionType,   // 108
    unk10c: [u8; PADDING_10C],     // 10C
}

unsafe impl ScriptClass for ChoiceEvent {
    type Kind = Native;
    const NAME: &'static str = "gameinteractionsChoiceEvent";
}

impl AsRef<InteractionBaseEvent> for ChoiceEvent {
    fn as_ref(&self) -> &InteractionBaseEvent {
        &self.base
    }
}

impl AsRef<Event> for ChoiceEvent {
    fn as_ref(&self) -> &Event {
        self.base.as_ref()
    }
}

impl AsRef<IScriptable> for ChoiceEvent {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ActionType {
    BUTTON_PRESSED = 0,
    BUTTON_RELEASED = 1,
    BUTTON_HOLD_PROGRESS = 2,
    BUTTON_HOLD_COMPLETE = 3,
    BUTTON_MULTITAP_BEGIN_LAST = 4,
    BUTTON_MULTITAP_END_LAST = 5,
    AXIS_CHANGE = 6,
    RELATIVE_CHANGE = 7,
    TOGGLE_PRESSED = 8,
    TOGGLE_RELEASED = 9,
    REPEAT = 10,
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                Self::BUTTON_PRESSED => "BUTTON_PRESSED",
                Self::BUTTON_RELEASED => "BUTTON_RELEASED",
                Self::BUTTON_HOLD_PROGRESS => "BUTTON_HOLD_PROGRESS",
                Self::BUTTON_HOLD_COMPLETE => "BUTTON_HOLD_COMPLETE",
                Self::BUTTON_MULTITAP_BEGIN_LAST => "BUTTON_MULTITAP_BEGIN_LAST",
                Self::BUTTON_MULTITAP_END_LAST => "BUTTON_MULTITAP_END_LAST",
                Self::AXIS_CHANGE => "AXIS_CHANGE",
                Self::RELATIVE_CHANGE => "RELATIVE_CHANGE",
                Self::TOGGLE_PRESSED => "TOGGLE_PRESSED",
                Self::TOGGLE_RELEASED => "TOGGLE_RELEASED",
                Self::REPEAT => "REPEAT",
            }
        )
    }
}

const PADDING_70: usize = 0x78 - 0x70;
const PADDING_98: usize = 0x9A - 0x98;
const PADDING_9B: usize = 0xA0 - 0x9B;

#[repr(C)]
pub struct InteractionChoice {
    pub caption: RedString,                          // 00
    pub caption_parts: InteractionChoiceCaption,     // 20
    pub data: RedArray<Variant>,                     // 30
    pub choice_meta_data: InteractionChoiceMetaData, // 40
    unk70: [u8; PADDING_70],                         // 70
    pub look_at_descriptor: ChoiceLookAtDescriptor,  // 78
    unk98: [u8; PADDING_98],                         // 98
    pub do_not_turn_off_prevention_system: bool,     // 9A
    unk9b: [u8; PADDING_9B],                         // 9B
}

unsafe impl ScriptClass for InteractionChoice {
    type Kind = Native;
    const NAME: &'static str = "gameinteractionsChoice";
}

impl fmt::Debug for InteractionChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Choice")
            .field("caption", &self.caption)
            .field("caption_parts", &self.caption_parts)
            .field("data", &self.data.len())
            .field("choice_meta_data", &self.choice_meta_data)
            .field("look_at_descriptor", &self.look_at_descriptor)
            .finish()
    }
}

const PADDING_04: usize = 0x8 - 0x4;

#[derive(Debug)]
#[repr(C)]
pub struct ChoiceLookAtDescriptor {
    pub type_: ChoiceLookAtType, // 00
    unk04: [u8; PADDING_04],     // 4
    pub slot_name: CName,        // 8
    pub offset: Vector3,         // 10
    pub orb_id: OrbID,           // 1C
}

unsafe impl NativeRepr for ChoiceLookAtDescriptor {
    const NAME: &'static str = "gameinteractionsChoiceLookAtDescriptor";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct OrbID(u32);

unsafe impl NativeRepr for OrbID {
    const NAME: &'static str = "gameinteractionsOrbID";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ChoiceLookAtType {
    Root = 0,
    Slot = 1,
    Orb = 2,
}

unsafe impl NativeRepr for ChoiceLookAtType {
    const NAME: &'static str = "gameinteractionsChoiceLookAtType";
}

const PADDING_2C: usize = 0x30 - 0x2C;

#[derive(Debug)]
#[repr(C)]
pub struct InteractionChoiceMetaData {
    pub tweak_db_name: RedString, // 00
    pub tweak_db_id: TweakDbId,   // 20
    pub type_: ChoiceTypeWrapper, // 28
    pub unk2c: [u8; PADDING_2C],  // 2C
}

unsafe impl NativeRepr for InteractionChoiceMetaData {
    const NAME: &'static str = "gameinteractionsChoiceMetaData";
}

#[derive(Debug)]
#[repr(C)]
pub struct ChoiceTypeWrapper {
    pub properties: u32, // 00
}

unsafe impl NativeRepr for ChoiceTypeWrapper {
    const NAME: &'static str = "gameinteractionsChoiceTypeWrapper";
}

#[derive(Debug)]
#[repr(C)]
pub struct InteractionChoiceCaption {
    pub parts: RedArray<ChoiceCaptionPart>,
}

unsafe impl NativeRepr for InteractionChoiceCaption {
    const NAME: &'static str = "gameinteractionsChoiceCaption";
}

#[derive(Debug)]
#[repr(transparent)]
pub struct ChoiceCaptionPart(IScriptable);

unsafe impl ScriptClass for ChoiceCaptionPart {
    type Kind = Native;
    const NAME: &'static str = "gameinteractionsChoiceCaptionPart";
}
