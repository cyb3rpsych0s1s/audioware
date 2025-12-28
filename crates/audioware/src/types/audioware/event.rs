use std::cell::{Cell, RefCell};

use red4ext_rs::{
    ScriptClass,
    class_kind::Native,
    types::{CName, EntityId, IScriptable, ResRef},
};

use crate::{
    AudParam, AudSwitch, ESoundCurveType, EventName, TFlag, Vector4, WwiseId,
    abi::callback::{
        FireAddContainerStreamingPrefetchCallback, FirePlayCallback, FirePlayExternalCallback,
        FirePlayOneShotCallback, FireRemoveContainerStreamingPrefetchCallback,
        FireSetAppearanceNameCallback, FireSetEntityNameCallback, FireSetGlobalParameterCallback,
        FireSetParameterCallback, FireSetSwitchCallback, FireStopCallback, FireStopTaggedCallback,
        FireTagCallback, FireUntagCallback,
    },
};

pub trait Hydrate<T> {
    fn hydrate(&mut self, other: &T);
}

pub trait WithWwise {
    fn wwise_id(&self) -> WwiseId;
}

pub trait WithEntityId {
    fn entity_id(&self) -> EntityId;
}

pub trait WithEmitter: WithEntityId {
    fn emitter_name(&self) -> CName;
}

pub trait WithPosition {
    fn position(&self) -> Vector4;
    fn has_position(&self) -> bool;
}

pub trait WithTags {
    fn sound_tags(&self) -> Vec<CName>;
    fn emitter_tags(&self) -> Vec<CName>;
}

pub trait WithOcclusions {
    fn graph_occlusion(&self) -> f32;
    fn raycast_occlusion(&self) -> f32;
    fn has_graph_occlusion(&self) -> bool;
    fn has_raycast_occlusion(&self) -> bool;
    fn is_in_different_room(&self) -> bool;
}

pub trait WithEventName {
    fn event_name(&self) -> CName;
}

pub trait WithName {
    fn name(&self) -> CName;
}

pub trait WithSeek {
    fn seek(&self) -> f32;
}

pub trait WithTagName {
    fn tag_name(&self) -> CName;
}

pub trait WithFloatData {
    fn float_data(&self) -> f32;
}

pub trait WithExternalResourcePath {
    fn external_resource_path(&self) -> u64;
}

pub trait WithParamsAndSwitches {
    fn params(&self) -> Vec<AudParam>;
    fn switches(&self) -> Vec<AudSwitch>;
}

macro_rules! with_entity_id {
    ($ty:ty) => {
        impl WithEntityId for $ty {
            fn entity_id(&self) -> EntityId {
                self.entity_id.get()
            }
        }
    };
    (base $ty:ty) => {
        impl WithEntityId for $ty {
            fn entity_id(&self) -> EntityId {
                self.base.entity_id()
            }
        }
    };
}

macro_rules! with_event_name {
    ($ty:ty) => {
        impl WithEventName for $ty {
            fn event_name(&self) -> CName {
                *self.event_name.get()
            }
        }
    };
    (base $ty:ty) => {
        impl WithEventName for $ty {
            fn event_name(&self) -> CName {
                self.base.event_name()
            }
        }
    };
}

macro_rules! with_name {
    ($ty:ty) => {
        impl WithName for $ty {
            fn name(&self) -> CName {
                self.name.get()
            }
        }
    };
    (base $ty:ty) => {
        impl WithName for $ty {
            fn name(&self) -> CName {
                self.base.name()
            }
        }
    };
}

macro_rules! with_tag_name {
    ($ty:ty) => {
        impl WithTagName for $ty {
            fn tag_name(&self) -> CName {
                self.tag_name.get()
            }
        }
    };
    (base $ty:ty) => {
        impl WithTagName for $ty {
            fn tag_name(&self) -> CName {
                self.base.tag_name()
            }
        }
    };
}

macro_rules! with_float_data {
    ($ty:ty) => {
        impl WithFloatData for $ty {
            fn float_data(&self) -> f32 {
                self.float_data.get()
            }
        }
    };
    (base $ty:ty) => {
        impl WithFloatData for $ty {
            fn float_data(&self) -> f32 {
                self.base.float_data()
            }
        }
    };
}

macro_rules! with_wwise {
    ($ty:ty) => {
        impl WithWwise for $ty {
            fn wwise_id(&self) -> WwiseId {
                self.base.wwise_id()
            }
        }
    };
}

with_wwise!(EngineEmitterEvent);
with_wwise!(PlayEvent);
with_wwise!(PlayExternalEvent);
with_wwise!(PlayOneShotEvent);

with_entity_id!(EngineEmitterEvent);
with_entity_id!(base PlayEvent);
with_entity_id!(base PlayExternalEvent);
with_entity_id!(base PlayOneShotEvent);
with_entity_id!(StopSoundEvent);
with_entity_id!(StopTaggedEvent);
with_entity_id!(AddContainerStreamingPrefetchEvent);
with_entity_id!(RemoveContainerStreamingPrefetchEvent);
with_entity_id!(TagEvent);
with_entity_id!(UntagEvent);
with_entity_id!(SetAppearanceNameEvent);
with_entity_id!(SetEntityNameEvent);

with_name!(SetAppearanceNameEvent);
with_name!(SetEntityNameEvent);

with_tag_name!(TagEvent);
with_tag_name!(UntagEvent);

with_event_name!(PlayEvent);
with_event_name!(base PlayExternalEvent);
with_event_name!(base PlayOneShotEvent);
with_event_name!(StopSoundEvent);
with_event_name!(AddContainerStreamingPrefetchEvent);
with_event_name!(RemoveContainerStreamingPrefetchEvent);

with_float_data!(StopSoundEvent);

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct EngineSoundEvent {
    base: IScriptable,
}

unsafe impl ScriptClass for EngineSoundEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.SoundEvent";
}

impl AsRef<IScriptable> for EngineSoundEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct EngineWwiseEvent {
    base: EngineSoundEvent,
    wwise_id: Cell<WwiseId>,
}

impl WithWwise for EngineWwiseEvent {
    fn wwise_id(&self) -> WwiseId {
        self.wwise_id.get()
    }
}

unsafe impl ScriptClass for EngineWwiseEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.WwiseEvent";
}

impl AsRef<IScriptable> for EngineWwiseEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base
    }
}

macro_rules! hydrate_from_wwise {
    ($ty:ty) => {
        impl Hydrate<$ty> for EngineWwiseEvent {
            fn hydrate(&mut self, other: &$ty) {
                self.wwise_id.set(other.wwise_id);
            }
        }
    };
}

hydrate_from_wwise!(FirePlayCallback);
hydrate_from_wwise!(FireSetAppearanceNameCallback);
hydrate_from_wwise!(FireSetEntityNameCallback);
hydrate_from_wwise!(FireStopCallback);
hydrate_from_wwise!(FireStopTaggedCallback);
hydrate_from_wwise!(FireTagCallback);
hydrate_from_wwise!(FireUntagCallback);
hydrate_from_wwise!(FireAddContainerStreamingPrefetchCallback);
hydrate_from_wwise!(FireRemoveContainerStreamingPrefetchCallback);

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct EngineEmitterEvent {
    base: EngineWwiseEvent,
    entity_id: Cell<EntityId>,
    emitter_name: Cell<CName>,
}

unsafe impl ScriptClass for EngineEmitterEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.EmitterEvent";
}

impl AsRef<IScriptable> for EngineEmitterEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base
    }
}

macro_rules! hydrate_from_emitter {
    ($ty:ty) => {
        impl Hydrate<$ty> for EngineEmitterEvent {
            fn hydrate(&mut self, other: &$ty) {
                self.base.hydrate(other);
                self.entity_id.set(other.entity_id);
                self.emitter_name.set(other.emitter_name);
            }
        }
    };
}

hydrate_from_emitter!(FirePlayCallback);

macro_rules! with_emitter {
    ($ty:ty) => {
        impl WithEmitter for $ty {
            fn emitter_name(&self) -> CName {
                self.emitter_name.get()
            }
        }
    };
    (base $ty:ty) => {
        impl WithEmitter for $ty {
            fn emitter_name(&self) -> CName {
                self.base.emitter_name()
            }
        }
    };
}

with_emitter!(EngineEmitterEvent);
with_emitter!(base PlayEvent);
with_emitter!(base PlayExternalEvent);
with_emitter!(base PlayOneShotEvent);

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PlayEvent {
    base: EngineEmitterEvent,
    event_name: Cell<EventName>,
    sound_tags: RefCell<Vec<CName>>,
    emitter_tags: RefCell<Vec<CName>>,
    seek: Cell<f32>,
    position: Cell<Vector4>,
    has_position: Cell<bool>,
}

impl WithPosition for PlayEvent {
    fn position(&self) -> Vector4 {
        self.position.get()
    }

    fn has_position(&self) -> bool {
        self.has_position.get()
    }
}

impl WithTags for PlayEvent {
    fn sound_tags(&self) -> Vec<CName> {
        self.sound_tags.borrow().clone()
    }

    fn emitter_tags(&self) -> Vec<CName> {
        self.emitter_tags.borrow().clone()
    }
}

impl WithSeek for PlayEvent {
    fn seek(&self) -> f32 {
        self.seek.get()
    }
}

unsafe impl ScriptClass for PlayEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.PlayEvent";
}

impl AsRef<IScriptable> for PlayEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base.base
    }
}

impl Hydrate<FirePlayCallback> for PlayEvent {
    fn hydrate(&mut self, other: &FirePlayCallback) {
        self.base.hydrate(other);
        self.event_name.set(other.event_name);
        *self.sound_tags.borrow_mut() = other.sound_tags.clone();
        *self.emitter_tags.borrow_mut() = other.emitter_tags.clone();
        self.seek.set(other.seek);
        self.position.set(other.position);
        self.has_position.set(other.has_position);
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PlayExternalEvent {
    base: PlayEvent,
    external_resource_path: RefCell<ResRef>,
}

impl WithExternalResourcePath for PlayExternalEvent {
    fn external_resource_path(&self) -> u64 {
        unsafe { std::mem::transmute::<ResRef, u64>(self.external_resource_path.borrow().clone()) }
    }
}

unsafe impl ScriptClass for PlayExternalEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.PlayExternalEvent";
}

impl AsRef<IScriptable> for PlayExternalEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base.base.base
    }
}

impl Hydrate<FirePlayExternalCallback> for PlayExternalEvent {
    fn hydrate(&mut self, other: &FirePlayExternalCallback) {
        self.base.hydrate(&other.base);
        self.external_resource_path
            .borrow_mut()
            .clone_from(&other.external_resource_path);
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PlayOneShotEvent {
    base: PlayEvent,
    params: RefCell<Vec<AudParam>>,
    switches: RefCell<Vec<AudSwitch>>,
    graph_occlusion: Cell<f32>,
    raycast_occlusion: Cell<f32>,
    flags: Cell<TFlag>,
}

impl WithParamsAndSwitches for PlayOneShotEvent {
    fn params(&self) -> Vec<AudParam> {
        self.params.borrow().clone()
    }

    fn switches(&self) -> Vec<AudSwitch> {
        self.switches.borrow().clone()
    }
}

impl WithOcclusions for PlayOneShotEvent {
    fn graph_occlusion(&self) -> f32 {
        self.graph_occlusion.get()
    }

    fn raycast_occlusion(&self) -> f32 {
        self.raycast_occlusion.get()
    }

    fn has_graph_occlusion(&self) -> bool {
        self.flags.get().contains(TFlag::HAS_GRAPH_OCCLUSION)
    }

    fn has_raycast_occlusion(&self) -> bool {
        self.flags.get().contains(TFlag::HAS_RAYCAST_OCCLUSION)
    }

    fn is_in_different_room(&self) -> bool {
        self.flags.get().contains(TFlag::IS_IN_DIFFERENT_ROOM)
    }
}

unsafe impl ScriptClass for PlayOneShotEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.PlayOneShotEvent";
}

impl AsRef<IScriptable> for PlayOneShotEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base.base.base
    }
}

impl Hydrate<FirePlayOneShotCallback> for PlayOneShotEvent {
    fn hydrate(&mut self, other: &FirePlayOneShotCallback) {
        self.base.hydrate(&other.base);
        *self.params.borrow_mut() = other
            .params
            .iter()
            .copied()
            .map(|x| AudParam {
                name: x.name,
                value: x.value,
            })
            .collect();
        *self.switches.borrow_mut() = other
            .switches
            .iter()
            .copied()
            .map(|x| AudSwitch {
                name: x.name,
                value: x.value,
            })
            .collect();
        self.graph_occlusion.set(other.graph_occlusion);
        self.raycast_occlusion.set(other.raycast_occlusion);
        self.flags.set(other.flags);
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct StopSoundEvent {
    base: EngineWwiseEvent,
    entity_id: Cell<EntityId>,
    event_name: Cell<EventName>,
    float_data: Cell<f32>,
}

unsafe impl ScriptClass for StopSoundEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.StopSoundEvent";
}

impl AsRef<IScriptable> for StopSoundEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base
    }
}

impl Hydrate<FireStopCallback> for StopSoundEvent {
    fn hydrate(&mut self, other: &FireStopCallback) {
        self.base.hydrate(other);
        self.entity_id.set(other.entity_id);
        self.event_name.set(other.event_name);
        self.float_data.set(other.float_data);
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct StopTaggedEvent {
    base: EngineWwiseEvent,
    entity_id: Cell<EntityId>,
    tag_name: Cell<CName>,
}

impl StopTaggedEvent {
    pub fn tag_name(&self) -> CName {
        self.tag_name.get()
    }
}

unsafe impl ScriptClass for StopTaggedEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.StopTaggedEvent";
}

impl AsRef<IScriptable> for StopTaggedEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base
    }
}

impl Hydrate<FireStopTaggedCallback> for StopTaggedEvent {
    fn hydrate(&mut self, other: &FireStopTaggedCallback) {
        self.base.hydrate(other);
        self.entity_id.set(other.entity_id);
        self.tag_name.set(*other.event_name);
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SetParameterEvent {
    base: EngineEmitterEvent,
    name_data: Cell<CName>,
    float_data: Cell<f32>,
}

impl SetParameterEvent {
    pub fn name_data(&self) -> CName {
        self.name_data.get()
    }
    pub fn float_data(&self) -> f32 {
        self.float_data.get()
    }
}

unsafe impl ScriptClass for SetParameterEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.SetParameterEvent";
}

impl AsRef<IScriptable> for SetParameterEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base.base
    }
}

impl Hydrate<FireSetParameterCallback> for SetParameterEvent {
    fn hydrate(&mut self, other: &FireSetParameterCallback) {
        self.base.hydrate(&other.base);
        self.name_data.set(other.switch_name);
        self.float_data.set(other.switch_value);
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SetGlobalParameterEvent {
    base: EngineWwiseEvent,
    name: Cell<EventName>,
    value: Cell<f32>,
    duration: Cell<f32>,
    curve_type: Cell<ESoundCurveType>,
}

impl SetGlobalParameterEvent {
    pub fn name(&self) -> CName {
        *self.name.get()
    }
    pub fn value(&self) -> f32 {
        self.value.get()
    }
    pub fn duration(&self) -> f32 {
        self.duration.get()
    }
    pub fn curve_type(&self) -> ESoundCurveType {
        self.curve_type.get()
    }
}

unsafe impl ScriptClass for SetGlobalParameterEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.SetGlobalParameterEvent";
}

impl AsRef<IScriptable> for SetGlobalParameterEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base
    }
}

impl Hydrate<FireSetGlobalParameterCallback> for SetGlobalParameterEvent {
    fn hydrate(&mut self, other: &FireSetGlobalParameterCallback) {
        self.base.wwise_id.set(other.wwise_id); // TODO
        self.name.set(other.name);
        self.value.set(other.value);
        self.duration.set(other.duration);
        self.curve_type.set(other.curve_type);
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SetSwitchEvent {
    base: EngineSoundEvent,
    switch_name: Cell<CName>,
    switch_value: Cell<CName>,
    switch_name_wwise_id: Cell<WwiseId>,
    switch_value_wwise_id: Cell<WwiseId>,
}

impl SetSwitchEvent {
    pub fn switch_name(&self) -> CName {
        self.switch_name.get()
    }
    pub fn switch_value(&self) -> CName {
        self.switch_value.get()
    }
    pub fn switch_name_wwise_id(&self) -> WwiseId {
        self.switch_name_wwise_id.get()
    }
    pub fn switch_value_wwise_id(&self) -> WwiseId {
        self.switch_value_wwise_id.get()
    }
}

unsafe impl ScriptClass for SetSwitchEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.SetSwitchEvent";
}

impl AsRef<IScriptable> for SetSwitchEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base
    }
}

impl Hydrate<FireSetSwitchCallback> for SetSwitchEvent {
    fn hydrate(&mut self, other: &FireSetSwitchCallback) {
        self.switch_name.set(other.switch_name);
        self.switch_value.set(other.switch_value);
        self.switch_name_wwise_id.set(other.switch_name_wwise_id);
        self.switch_value_wwise_id.set(other.switch_value_wwise_id);
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SetAppearanceNameEvent {
    base: EngineWwiseEvent,
    entity_id: Cell<EntityId>,
    name: Cell<CName>,
}

impl SetAppearanceNameEvent {
    pub fn name(&self) -> CName {
        self.name.get()
    }
}

unsafe impl ScriptClass for SetAppearanceNameEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.SetAppearanceNameEvent";
}

impl AsRef<IScriptable> for SetAppearanceNameEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base
    }
}

impl Hydrate<FireSetAppearanceNameCallback> for SetAppearanceNameEvent {
    fn hydrate(&mut self, other: &FireSetAppearanceNameCallback) {
        self.base.hydrate(other);
        self.entity_id.set(other.entity_id);
        self.name.set(*other.event_name);
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SetEntityNameEvent {
    base: EngineWwiseEvent,
    entity_id: Cell<EntityId>,
    name: Cell<CName>,
}

impl SetEntityNameEvent {
    pub fn name(&self) -> CName {
        self.name.get()
    }
}

unsafe impl ScriptClass for SetEntityNameEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.SetEntityNameEvent";
}

impl AsRef<IScriptable> for SetEntityNameEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base
    }
}

impl Hydrate<FireSetEntityNameCallback> for SetEntityNameEvent {
    fn hydrate(&mut self, other: &FireSetEntityNameCallback) {
        self.base.hydrate(other);
        self.entity_id.set(other.entity_id);
        self.name.set(*other.event_name);
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct TagEvent {
    base: EngineWwiseEvent,
    entity_id: Cell<EntityId>,
    tag_name: Cell<CName>,
}

unsafe impl ScriptClass for TagEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.TagEvent";
}

impl AsRef<IScriptable> for TagEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base
    }
}

impl Hydrate<FireTagCallback> for TagEvent {
    fn hydrate(&mut self, other: &FireTagCallback) {
        self.base.hydrate(other);
        self.entity_id.set(other.entity_id);
        self.tag_name.set(*other.event_name);
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct UntagEvent {
    base: EngineWwiseEvent,
    entity_id: Cell<EntityId>,
    tag_name: Cell<CName>,
}

unsafe impl ScriptClass for UntagEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.UntagEvent";
}

impl AsRef<IScriptable> for UntagEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base
    }
}

impl Hydrate<FireUntagCallback> for UntagEvent {
    fn hydrate(&mut self, other: &FireUntagCallback) {
        self.base.hydrate(other);
        self.entity_id.set(other.entity_id);
        self.tag_name.set(*other.event_name);
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct AddContainerStreamingPrefetchEvent {
    base: EngineWwiseEvent,
    entity_id: Cell<EntityId>,
    event_name: Cell<EventName>,
}

unsafe impl ScriptClass for AddContainerStreamingPrefetchEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.AddContainerStreamingPrefetchEvent";
}

impl AsRef<IScriptable> for AddContainerStreamingPrefetchEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base
    }
}

impl Hydrate<FireAddContainerStreamingPrefetchCallback> for AddContainerStreamingPrefetchEvent {
    fn hydrate(&mut self, other: &FireAddContainerStreamingPrefetchCallback) {
        self.base.hydrate(other);
        self.entity_id.set(other.entity_id);
        self.event_name.set(other.event_name);
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct RemoveContainerStreamingPrefetchEvent {
    base: EngineWwiseEvent,
    entity_id: Cell<EntityId>,
    event_name: Cell<EventName>,
}

unsafe impl ScriptClass for RemoveContainerStreamingPrefetchEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.RemoveContainerStreamingPrefetchEvent";
}

impl AsRef<IScriptable> for RemoveContainerStreamingPrefetchEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base
    }
}

impl Hydrate<FireRemoveContainerStreamingPrefetchCallback>
    for RemoveContainerStreamingPrefetchEvent
{
    fn hydrate(&mut self, other: &FireRemoveContainerStreamingPrefetchCallback) {
        self.base.hydrate(other);
        self.entity_id.set(other.entity_id);
        self.event_name.set(other.event_name);
    }
}
