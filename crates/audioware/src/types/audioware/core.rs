use std::ops::Deref;

use red4ext_rs::{
    ScriptClass,
    class_kind::Native,
    types::{CName, EntityId, IScriptable, Ref},
};

use crate::EventActionType;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct EventName(CName);

impl TryFrom<CName> for EventName {
    type Error = ();

    fn try_from(value: CName) -> Result<Self, Self::Error> {
        if value == CName::undefined() {
            return Err(());
        }
        Ok(Self(value))
    }
}

impl std::fmt::Display for EventName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl Deref for EventName {
    type Target = CName;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ClassName(CName);

impl TryFrom<CName> for ClassName {
    type Error = ();

    fn try_from(value: CName) -> Result<Self, Self::Error> {
        if value == CName::undefined() {
            return Err(());
        }
        Ok(Self(value))
    }
}

impl std::fmt::Display for ClassName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl Deref for ClassName {
    type Target = CName;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FunctionName(CName);

impl TryFrom<CName> for FunctionName {
    type Error = ();

    fn try_from(value: CName) -> Result<Self, Self::Error> {
        if value == CName::undefined() {
            return Err(());
        }
        Ok(Self(value))
    }
}

impl std::fmt::Display for FunctionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl Deref for FunctionName {
    type Target = CName;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct AudioEventCallbackTarget {
    base: IScriptable,
}

unsafe impl ScriptClass for AudioEventCallbackTarget {
    type Kind = Native;
    const NAME: &'static str = "Audioware.AudioEventCallbackTarget";
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct AudioEventCallbackEntityTarget {
    base: AudioEventCallbackTarget,
    pub value: Option<EntityTargetInner>,
}

impl AudioEventCallbackEntityTarget {
    pub fn new_with_entity_id(value: EntityId) -> Ref<Self> {
        Ref::new_with(|x: &mut Self| {
            x.value = Some(EntityTargetInner::Id(value));
        })
        .unwrap_or_default()
    }
    pub fn new_with_emitter_name(value: CName) -> Ref<Self> {
        Ref::new_with(|x: &mut Self| {
            x.value = Some(EntityTargetInner::EmitterName(value));
        })
        .unwrap_or_default()
    }
}

unsafe impl ScriptClass for AudioEventCallbackEntityTarget {
    type Kind = Native;
    const NAME: &'static str = "Audioware.EntityTarget";
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum EntityTargetInner {
    Id(EntityId),
    EmitterName(CName),
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct AudioEventCallbackEventTarget {
    base: AudioEventCallbackTarget,
    pub value: Option<EventTargetInner>,
}

impl AudioEventCallbackEventTarget {
    pub fn new_with_action_type(value: EventActionType) -> Ref<Self> {
        Ref::new_with(|x: &mut Self| {
            x.value = Some(EventTargetInner::ActionType(value));
        })
        .unwrap_or_default()
    }
    pub fn new_with_action_type_name(value: String) -> Ref<Self> {
        let ty = match value.as_str() {
            "Play" => Some(EventActionType::Play),
            "PlayAnimation" => Some(EventActionType::PlayAnimation),
            "SetParameter" => Some(EventActionType::SetParameter),
            "StopSound" => Some(EventActionType::StopSound),
            "SetSwitch" => Some(EventActionType::SetSwitch),
            "StopTagged" => Some(EventActionType::StopTagged),
            "PlayExternal" => Some(EventActionType::PlayExternal),
            "Tag" => Some(EventActionType::Tag),
            "Untag" => Some(EventActionType::Untag),
            "SetAppearanceName" => Some(EventActionType::SetAppearanceName),
            "SetEntityName" => Some(EventActionType::SetEntityName),
            "AddContainerStreamingPrefetch" => Some(EventActionType::AddContainerStreamingPrefetch),
            "RemoveContainerStreamingPrefetch" => {
                Some(EventActionType::RemoveContainerStreamingPrefetch)
            }
            _ => None,
        };
        if let Some(ty) = ty {
            return Self::new_with_action_type(ty);
        }
        Ref::default()
    }
}

unsafe impl ScriptClass for AudioEventCallbackEventTarget {
    type Kind = Native;
    const NAME: &'static str = "Audioware.EventTarget";
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum EventTargetInner {
    ActionType(EventActionType),
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum AnyTarget {
    Id(EntityId),
    EmitterName(CName),
    Type(EventActionType),
}
