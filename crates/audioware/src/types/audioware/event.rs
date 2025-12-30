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

macro_rules! getter {
    (cell $name:ident -> $ty:ty) => {
        pub fn $name(&self) -> $ty {
            self.$name.get()
        }
    };
    (*cell $name:ident -> $ty:ty) => {
        pub fn $name(&self) -> $ty {
            *self.$name.get()
        }
    };
    (refcell $name:ident -> $ty:ty) => {
        pub fn $name(&self) -> $ty {
            self.$name.borrow().clone()
        }
    };
}

macro_rules! setter {
    (cell $set:ident $name:ident -> $ty:ty => $other:ty) => {
        pub fn $set(&self, other: &$other) {
            self.$name.set(other.$name);
        }
    };
    (*cell $set:ident $name:ident -> $ty:ty => $other:ty) => {
        pub fn $set(&self, other: &$other) {
            self.$name.set(*other.$name);
        }
    };
    (refcell $set:ident $name:ident -> $ty:ty => $other:ty) => {
        pub fn $set(&self, other: &$other) {
            *self.$name.borrow_mut() = other.$name.clone();
        }
    };
    (refcell cast $elem:ident, $set:ident $name:ident -> $ty:ty => $other:ty) => {
        pub fn $set(&self, other: &$other) {
            *self.$name.borrow_mut() = other
                .$name
                .iter()
                .copied()
                .map(|x| $elem {
                    name: x.name,
                    value: x.value,
                })
                .collect();
        }
    };
    (base cell $set:ident $name:ident -> $ty:ty => $other:ty) => {
        #[inline]
        pub fn $set(&self, other: &$other) {
            self.$name.set(other.base.$name);
        }
    };
    (base *cell $set:ident $name:ident -> $ty:ty => $other:ty) => {
        #[inline]
        pub fn $set(&self, other: &$other) {
            self.$name.set(*other.base.$name);
        }
    };
    (base refcell $set:ident $name:ident -> $ty:ty => $other:ty) => {
        #[inline]
        pub fn $set(&self, other: &$other) {
            *self.$name.borrow_mut() = other.base.$name.clone();
        }
    };
}

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
pub struct PlayEvent {
    base: EngineSoundEvent,
    event_name: Cell<EventName>,
    entity_id: Cell<EntityId>,
    emitter_name: Cell<CName>,
    position: Cell<Vector4>,
    wwise_id: Cell<WwiseId>,
    sound_tags: RefCell<Vec<CName>>,
    emitter_tags: RefCell<Vec<CName>>,
    seek: Cell<f32>,
}

impl PlayEvent {
    getter!(*cell event_name -> CName);
    getter!(cell entity_id -> EntityId);
    getter!(cell emitter_name -> CName);
    getter!(cell position -> Vector4);
    getter!(cell wwise_id -> WwiseId);
    getter!(refcell sound_tags -> Vec<CName>);
    getter!(refcell emitter_tags -> Vec<CName>);
    getter!(cell seek -> f32);

    setter!(cell set_event_name event_name -> CName => FirePlayCallback);
    setter!(cell set_entity_id entity_id -> EntityId => FirePlayCallback);
    setter!(cell set_emitter_name emitter_name -> CName => FirePlayCallback);
    setter!(cell set_position position -> Vector4 => FirePlayCallback);
    setter!(cell set_wwise_id wwise_id -> WwiseId => FirePlayCallback);
    setter!(refcell set_sound_tags sound_tags -> Vec<CName> => FirePlayCallback);
    setter!(refcell set_emitter_tags emitter_tags -> Vec<CName> => FirePlayCallback);
    setter!(cell set_seek seek -> f32 => FirePlayCallback);

    pub fn hydrate(&mut self, other: &FirePlayCallback) {
        self.set_event_name(other);
        self.set_entity_id(other);
        self.set_emitter_name(other);
        self.set_position(other);
        self.set_wwise_id(other);
        self.set_sound_tags(other);
        self.set_emitter_tags(other);
        self.set_seek(other);
    }
}

unsafe impl ScriptClass for PlayEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.PlayEvent";
}

impl AsRef<IScriptable> for PlayEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PlayExternalEvent {
    base: EngineSoundEvent,
    event_name: Cell<EventName>,
    entity_id: Cell<EntityId>,
    emitter_name: Cell<CName>,
    wwise_id: Cell<WwiseId>,
    sound_tags: RefCell<Vec<CName>>,
    emitter_tags: RefCell<Vec<CName>>,
    seek: Cell<f32>,
    position: Cell<Vector4>,
    external_resource_path: RefCell<ResRef>,
}

impl PlayExternalEvent {
    getter!(*cell event_name -> CName);
    getter!(cell entity_id -> EntityId);
    getter!(cell emitter_name -> CName);
    getter!(cell position -> Vector4);
    getter!(cell wwise_id -> WwiseId);
    getter!(refcell sound_tags -> Vec<CName>);
    getter!(refcell emitter_tags -> Vec<CName>);
    getter!(cell seek -> f32);

    pub fn external_resource_path(&self) -> u64 {
        unsafe { std::mem::transmute::<ResRef, u64>(self.external_resource_path.borrow().clone()) }
    }

    setter!(base cell set_event_name event_name -> CName => FirePlayExternalCallback);
    setter!(base cell set_entity_id entity_id -> EntityId => FirePlayExternalCallback);
    setter!(base cell set_emitter_name emitter_name -> CName => FirePlayExternalCallback);
    setter!(base cell set_position position -> Vector4 => FirePlayExternalCallback);
    setter!(base cell set_wwise_id wwise_id -> WwiseId => FirePlayExternalCallback);
    setter!(base refcell set_sound_tags sound_tags -> Vec<CName> => FirePlayExternalCallback);
    setter!(base refcell set_emitter_tags emitter_tags -> Vec<CName> => FirePlayExternalCallback);
    setter!(base cell set_seek seek -> f32 => FirePlayExternalCallback);

    pub fn set_external_resource_path(&self, other: &FirePlayExternalCallback) {
        *self.external_resource_path.borrow_mut() = other.external_resource_path.clone();
    }

    pub fn hydrate(&mut self, other: &FirePlayExternalCallback) {
        self.set_event_name(other);
        self.set_entity_id(other);
        self.set_emitter_name(other);
        self.set_position(other);
        self.set_wwise_id(other);
        self.set_sound_tags(other);
        self.set_emitter_tags(other);
        self.set_seek(other);
        self.set_external_resource_path(other);
    }
}

unsafe impl ScriptClass for PlayExternalEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.PlayExternalEvent";
}

impl AsRef<IScriptable> for PlayExternalEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PlayOneShotEvent {
    base: EngineSoundEvent,
    event_name: Cell<EventName>,
    entity_id: Cell<EntityId>,
    emitter_name: Cell<CName>,
    position: Cell<Vector4>,
    wwise_id: Cell<WwiseId>,
    params: RefCell<Vec<AudParam>>,
    switches: RefCell<Vec<AudSwitch>>,
    graph_occlusion: Cell<f32>,
    raycast_occlusion: Cell<f32>,
    flags: Cell<TFlag>,
}

impl PlayOneShotEvent {
    getter!(*cell event_name -> CName);
    getter!(cell entity_id -> EntityId);
    getter!(cell emitter_name -> CName);
    getter!(cell position -> Vector4);
    getter!(cell wwise_id -> WwiseId);
    getter!(refcell params -> Vec<AudParam>);
    getter!(refcell switches -> Vec<AudSwitch>);
    getter!(cell graph_occlusion -> f32);
    getter!(cell raycast_occlusion -> f32);

    pub fn has_graph_occlusion(&self) -> bool {
        self.flags.get().contains(TFlag::HAS_GRAPH_OCCLUSION)
    }
    pub fn has_raycast_occlusion(&self) -> bool {
        self.flags.get().contains(TFlag::HAS_RAYCAST_OCCLUSION)
    }
    pub fn is_in_different_room(&self) -> bool {
        self.flags.get().contains(TFlag::IS_IN_DIFFERENT_ROOM)
    }

    setter!(base cell set_event_name event_name -> CName => FirePlayOneShotCallback);
    setter!(base cell set_entity_id entity_id -> EntityId => FirePlayOneShotCallback);
    setter!(base cell set_emitter_name emitter_name -> CName => FirePlayOneShotCallback);
    setter!(base cell set_position position -> Vector4 => FirePlayOneShotCallback);
    setter!(base cell set_wwise_id wwise_id -> WwiseId => FirePlayOneShotCallback);
    setter!(cell set_graph_occlusion graph_occlusion -> f32 => FirePlayOneShotCallback);
    setter!(cell set_raycast_occlusion raycast_occlusion -> f32 => FirePlayOneShotCallback);
    setter!(refcell cast AudParam, set_params params -> Vec<AudParam> => FirePlayOneShotCallback);
    setter!(refcell cast AudSwitch, set_switches switches -> Vec<AudSwitch> => FirePlayOneShotCallback);

    pub fn hydrate(&mut self, other: &FirePlayOneShotCallback) {
        self.set_event_name(other);
        self.set_entity_id(other);
        self.set_emitter_name(other);
        self.set_position(other);
        self.set_wwise_id(other);
        self.set_graph_occlusion(other);
        self.set_raycast_occlusion(other);
        self.set_params(other);
        self.set_switches(other);
    }
}

unsafe impl ScriptClass for PlayOneShotEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.PlayOneShotEvent";
}

impl AsRef<IScriptable> for PlayOneShotEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct StopSoundEvent {
    base: EngineSoundEvent,
    wwise_id: Cell<WwiseId>,
    entity_id: Cell<EntityId>,
    event_name: Cell<EventName>,
    float_data: Cell<f32>,
}

impl StopSoundEvent {
    getter!(*cell event_name -> CName);
    getter!(cell entity_id -> EntityId);
    getter!(cell wwise_id -> WwiseId);
    getter!(cell float_data -> f32);

    setter!(cell set_event_name event_name -> EventName => FireStopCallback);
    setter!(cell set_entity_id entity_id -> EntityId => FireStopCallback);
    setter!(cell set_wwise_id wwise_id -> WwiseId => FireStopCallback);
    setter!(cell set_float_data float_data -> f32 => FireStopCallback);

    pub fn hydrate(&mut self, other: &FireStopCallback) {
        self.set_event_name(other);
        self.set_entity_id(other);
        self.set_float_data(other);
        self.set_wwise_id(other);
    }
}

unsafe impl ScriptClass for StopSoundEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.StopSoundEvent";
}

impl AsRef<IScriptable> for StopSoundEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct StopTaggedEvent {
    base: EngineSoundEvent,
    tag_name: Cell<CName>,
    entity_id: Cell<EntityId>,
    wwise_id: Cell<WwiseId>,
}

impl StopTaggedEvent {
    getter!(cell tag_name -> CName);
    getter!(cell entity_id -> EntityId);
    getter!(cell wwise_id -> WwiseId);

    setter!(cell set_entity_id entity_id -> EntityId => FireStopTaggedCallback);
    setter!(cell set_wwise_id wwise_id -> WwiseId => FireStopTaggedCallback);

    pub fn set_tag_name(&mut self, other: &FireStopTaggedCallback) {
        self.tag_name.set(*other.event_name);
    }

    pub fn hydrate(&mut self, other: &FireStopTaggedCallback) {
        self.set_tag_name(other);
        self.set_entity_id(other);
        self.set_wwise_id(other);
    }
}

unsafe impl ScriptClass for StopTaggedEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.StopTaggedEvent";
}

impl AsRef<IScriptable> for StopTaggedEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SetParameterEvent {
    base: EngineSoundEvent,
    switch_name: Cell<CName>,
    switch_value: Cell<f32>,
    entity_id: Cell<EntityId>,
    emitter_name: Cell<CName>,
    position: Cell<Vector4>,
    wwise_id: Cell<WwiseId>,
    sound_tags: RefCell<Vec<CName>>,
    emitter_tags: RefCell<Vec<CName>>,
}

impl SetParameterEvent {
    getter!(cell switch_name -> CName);
    getter!(cell switch_value -> f32);
    getter!(cell entity_id -> EntityId);
    getter!(cell emitter_name -> CName);
    getter!(cell position -> Vector4);
    getter!(cell wwise_id -> WwiseId);
    getter!(refcell sound_tags -> Vec<CName>);
    getter!(refcell emitter_tags -> Vec<CName>);

    setter!(cell set_switch_name switch_name -> CName => FireSetParameterCallback);
    setter!(cell set_switch_value switch_value -> f32 => FireSetParameterCallback);
    setter!(base cell set_entity_id entity_id -> EntityId => FireSetParameterCallback);
    setter!(base cell set_emitter_name emitter_name -> CName => FireSetParameterCallback);
    setter!(base cell set_position position -> Vector4 => FireSetParameterCallback);
    setter!(base cell set_wwise_id wwise_id -> WwiseId => FireSetParameterCallback);
    setter!(base refcell set_sound_tags sound_tags -> Vec<CName> => FireSetParameterCallback);
    setter!(base refcell set_emitter_tags emitter_tags -> Vec<CName> => FireSetParameterCallback);

    pub fn hydrate(&mut self, other: &FireSetParameterCallback) {
        self.set_switch_name(other);
        self.set_switch_value(other);
        self.set_entity_id(other);
        self.set_emitter_name(other);
        self.set_position(other);
        self.set_wwise_id(other);
        self.set_sound_tags(other);
        self.set_emitter_tags(other);
    }
}

unsafe impl ScriptClass for SetParameterEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.SetParameterEvent";
}

impl AsRef<IScriptable> for SetParameterEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SetGlobalParameterEvent {
    base: EngineSoundEvent,
    name: Cell<EventName>,
    value: Cell<f32>,
    duration: Cell<f32>,
    curve_type: Cell<ESoundCurveType>,
    wwise_id: Cell<WwiseId>,
}

impl SetGlobalParameterEvent {
    getter!(*cell name -> CName);
    getter!(cell value -> f32);
    getter!(cell duration -> f32);
    getter!(cell curve_type -> ESoundCurveType);
    getter!(cell wwise_id -> WwiseId);

    setter!(cell set_name name -> CName => FireSetGlobalParameterCallback);
    setter!(cell set_value value -> f32 => FireSetGlobalParameterCallback);
    setter!(cell set_duration duration -> f32 => FireSetGlobalParameterCallback);
    setter!(cell set_curve_type curve_type -> ESoundCurveType => FireSetGlobalParameterCallback);
    setter!(cell set_wwise_id wwise_id -> WwiseId => FireSetGlobalParameterCallback);

    pub fn hydrate(&mut self, other: &FireSetGlobalParameterCallback) {
        self.set_name(other);
        self.set_value(other);
        self.set_duration(other);
        self.set_curve_type(other);
        self.set_wwise_id(other);
    }
}

unsafe impl ScriptClass for SetGlobalParameterEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.SetGlobalParameterEvent";
}

impl AsRef<IScriptable> for SetGlobalParameterEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base
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
    entity_id: Cell<EntityId>,
    emitter_name: Cell<CName>,
    position: Cell<Vector4>,
    sound_tags: RefCell<Vec<CName>>,
    emitter_tags: RefCell<Vec<CName>>,
}

impl SetSwitchEvent {
    getter!(cell switch_name -> CName);
    getter!(cell switch_value -> CName);
    getter!(cell switch_name_wwise_id -> WwiseId);
    getter!(cell switch_value_wwise_id -> WwiseId);
    getter!(cell entity_id -> EntityId);
    getter!(cell emitter_name -> CName);
    getter!(cell position -> Vector4);
    getter!(refcell sound_tags -> Vec<CName>);
    getter!(refcell emitter_tags -> Vec<CName>);

    setter!(cell set_switch_name switch_name -> CName => FireSetSwitchCallback);
    setter!(cell set_switch_value switch_value -> CName => FireSetSwitchCallback);
    setter!(cell set_switch_name_wwise_id switch_name_wwise_id -> WwiseId => FireSetSwitchCallback);
    setter!(cell set_switch_value_wwise_id switch_value_wwise_id -> WwiseId => FireSetSwitchCallback);
    setter!(base cell set_entity_id entity_id -> EntityId => FireSetSwitchCallback);
    setter!(base cell set_emitter_name emitter_name -> CName => FireSetSwitchCallback);
    setter!(base cell set_position position -> Vector4 => FireSetSwitchCallback);
    setter!(base refcell set_sound_tags sound_tags -> Vec<CName> => FireSetSwitchCallback);
    setter!(base refcell set_emitter_tags emitter_tags -> Vec<CName> => FireSetSwitchCallback);

    pub fn hydrate(&mut self, other: &FireSetSwitchCallback) {
        self.set_switch_name(other);
        self.set_switch_value(other);
        self.set_switch_name_wwise_id(other);
        self.set_switch_value_wwise_id(other);
        self.set_entity_id(other);
        self.set_emitter_name(other);
        self.set_position(other);
        self.set_sound_tags(other);
        self.set_emitter_tags(other);
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

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SetAppearanceNameEvent {
    base: EngineSoundEvent,
    event_name: Cell<CName>,
    entity_id: Cell<EntityId>,
    wwise_id: Cell<WwiseId>,
}

impl SetAppearanceNameEvent {
    getter!(cell event_name -> CName);
    getter!(cell entity_id -> EntityId);
    getter!(cell wwise_id -> WwiseId);

    setter!(*cell set_event_name event_name -> CName => FireSetAppearanceNameCallback);
    setter!(cell set_entity_id entity_id -> EntityId => FireSetAppearanceNameCallback);
    setter!(cell set_wwise_id wwise_id -> WwiseId => FireSetAppearanceNameCallback);

    pub fn hydrate(&mut self, other: &FireSetAppearanceNameCallback) {
        self.set_event_name(other);
        self.set_entity_id(other);
        self.set_wwise_id(other);
    }
}

unsafe impl ScriptClass for SetAppearanceNameEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.SetAppearanceNameEvent";
}

impl AsRef<IScriptable> for SetAppearanceNameEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SetEntityNameEvent {
    base: EngineSoundEvent,
    event_name: Cell<CName>,
    entity_id: Cell<EntityId>,
    wwise_id: Cell<WwiseId>,
}

impl SetEntityNameEvent {
    getter!(cell event_name -> CName);
    getter!(cell entity_id -> EntityId);
    getter!(cell wwise_id -> WwiseId);

    setter!(*cell set_event_name event_name -> CName => FireSetEntityNameCallback);
    setter!(cell set_entity_id entity_id -> EntityId => FireSetEntityNameCallback);
    setter!(cell set_wwise_id wwise_id -> WwiseId => FireSetEntityNameCallback);

    pub fn hydrate(&mut self, other: &FireSetEntityNameCallback) {
        self.set_event_name(other);
        self.set_entity_id(other);
        self.set_wwise_id(other);
    }
}

unsafe impl ScriptClass for SetEntityNameEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.SetEntityNameEvent";
}

impl AsRef<IScriptable> for SetEntityNameEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct TagEvent {
    base: EngineSoundEvent,
    event_name: Cell<CName>,
    entity_id: Cell<EntityId>,
    wwise_id: Cell<WwiseId>,
}

impl TagEvent {
    getter!(cell event_name -> CName);
    getter!(cell entity_id -> EntityId);
    getter!(cell wwise_id -> WwiseId);

    setter!(*cell set_event_name event_name -> CName => FireTagCallback);
    setter!(cell set_entity_id entity_id -> EntityId => FireTagCallback);
    setter!(cell set_wwise_id wwise_id -> WwiseId => FireTagCallback);

    pub fn hydrate(&mut self, other: &FireTagCallback) {
        self.set_event_name(other);
        self.set_entity_id(other);
        self.set_wwise_id(other);
    }
}

unsafe impl ScriptClass for TagEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.TagEvent";
}

impl AsRef<IScriptable> for TagEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct UntagEvent {
    base: EngineSoundEvent,
    event_name: Cell<CName>,
    entity_id: Cell<EntityId>,
    wwise_id: Cell<WwiseId>,
}

impl UntagEvent {
    getter!(cell event_name -> CName);
    getter!(cell entity_id -> EntityId);
    getter!(cell wwise_id -> WwiseId);

    setter!(*cell set_event_name event_name -> CName => FireUntagCallback);
    setter!(cell set_entity_id entity_id -> EntityId => FireUntagCallback);
    setter!(cell set_wwise_id wwise_id -> WwiseId => FireUntagCallback);

    pub fn hydrate(&mut self, other: &FireUntagCallback) {
        self.set_event_name(other);
        self.set_entity_id(other);
        self.set_wwise_id(other);
    }
}

unsafe impl ScriptClass for UntagEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.UntagEvent";
}

impl AsRef<IScriptable> for UntagEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct AddContainerStreamingPrefetchEvent {
    base: EngineSoundEvent,
    event_name: Cell<EventName>,
    entity_id: Cell<EntityId>,
    wwise_id: Cell<WwiseId>,
}

impl AddContainerStreamingPrefetchEvent {
    getter!(*cell event_name -> CName);
    getter!(cell entity_id -> EntityId);
    getter!(cell wwise_id -> WwiseId);

    setter!(cell set_event_name event_name -> CName => FireAddContainerStreamingPrefetchCallback);
    setter!(cell set_entity_id entity_id -> EntityId => FireAddContainerStreamingPrefetchCallback);
    setter!(cell set_wwise_id wwise_id -> WwiseId => FireAddContainerStreamingPrefetchCallback);

    pub fn hydrate(&mut self, other: &FireAddContainerStreamingPrefetchCallback) {
        self.set_event_name(other);
        self.set_entity_id(other);
        self.set_wwise_id(other);
    }
}

unsafe impl ScriptClass for AddContainerStreamingPrefetchEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.AddContainerStreamingPrefetchEvent";
}

impl AsRef<IScriptable> for AddContainerStreamingPrefetchEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct RemoveContainerStreamingPrefetchEvent {
    base: EngineSoundEvent,
    wwise_id: Cell<WwiseId>,
    entity_id: Cell<EntityId>,
    event_name: Cell<EventName>,
}

impl RemoveContainerStreamingPrefetchEvent {
    getter!(*cell event_name -> CName);
    getter!(cell entity_id -> EntityId);
    getter!(cell wwise_id -> WwiseId);

    setter!(cell set_event_name event_name -> CName => FireRemoveContainerStreamingPrefetchCallback);
    setter!(cell set_entity_id entity_id -> EntityId => FireRemoveContainerStreamingPrefetchCallback);
    setter!(cell set_wwise_id wwise_id -> WwiseId => FireRemoveContainerStreamingPrefetchCallback);

    pub fn hydrate(&mut self, other: &FireRemoveContainerStreamingPrefetchCallback) {
        self.set_event_name(other);
        self.set_entity_id(other);
        self.set_wwise_id(other);
    }
}

unsafe impl ScriptClass for RemoveContainerStreamingPrefetchEvent {
    type Kind = Native;
    const NAME: &'static str = "Audioware.RemoveContainerStreamingPrefetchEvent";
}

impl AsRef<IScriptable> for RemoveContainerStreamingPrefetchEvent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base
    }
}
