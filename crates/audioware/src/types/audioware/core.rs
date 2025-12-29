use std::ops::Deref;

use red4ext_rs::{
    NativeRepr, ScriptClass,
    class_kind::Native,
    types::{CName, EntityId, IScriptable, Ref},
};

use crate::{EventActionType, WwiseId};

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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i64)]
pub enum EventHookType {
    #[default]
    Play = 0,
    PlayOneShot = 1,
    SetParameter = 2,
    StopSound = 3,
    SetSwitch = 4,
    StopTagged = 5,
    PlayExternal = 6,
    Tag = 7,
    Untag = 8,
    SetAppearanceName = 9,
    SetEntityName = 10,
    AddContainerStreamingPrefetch = 11,
    RemoveContainerStreamingPrefetch = 12,
    SetGlobalParameter = 13,
}

unsafe impl NativeRepr for EventHookType {
    const NAME: &'static str = "Audioware.EventHookType";
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

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct AudioEventCallbackAssetTarget {
    base: AudioEventCallbackTarget,
    pub value: Option<AssetTargetInner>,
}

impl AudioEventCallbackAssetTarget {
    pub fn new_with_wwise_id(value: WwiseId) -> Ref<Self> {
        Ref::new_with(|x: &mut Self| {
            x.value = Some(AssetTargetInner::WwiseId(value));
        })
        .unwrap_or_default()
    }
}

unsafe impl ScriptClass for AudioEventCallbackAssetTarget {
    type Kind = Native;
    const NAME: &'static str = "Audioware.AssetTarget";
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
    pub fn new_with_hook_type(value: EventHookType) -> Ref<Self> {
        Ref::new_with(|x: &mut Self| {
            x.value = Some(EventTargetInner::HookType(value));
        })
        .unwrap_or_default()
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
    HookType(EventHookType),
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum AssetTargetInner {
    WwiseId(WwiseId),
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum AnyTarget {
    Id(EntityId),
    EmitterName(CName),
    Type(EventActionType),
    Hook(EventHookType),
    Wwise(WwiseId),
}
