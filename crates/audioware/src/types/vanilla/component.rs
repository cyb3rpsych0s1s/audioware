use debug_ignore::DebugIgnore;
use red4ext_rs::{
    NativeRepr, RttiSystem, ScriptClass,
    class_kind::Native,
    types::{CName, Cruid, IScriptable, RedArray, Ref, WeakRef},
};
use std::mem;

use crate::{AudEventStruct, AudParameter, AudSwitch, RedTagList};

use super::{Entity, WorldTransform};

#[derive(Debug)]
#[repr(C)]
pub struct IComponent {
    pub base: IScriptable,
    pub name: CName, // 0x40
    pub _padding0: [u8; 0x18],
    pub id: Cruid, // 0x60
    pub _padding1: [u8; 0x23],
    pub is_enabled: bool,    // 0x8B
    pub is_replicable: bool, // 0x8C
}

unsafe impl ScriptClass for IComponent {
    const NAME: &'static str = "entIComponent";
    type Kind = Native;
}

impl AsRef<IScriptable> for IComponent {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}

pub trait AsIComponent {
    fn get_entity(&self) -> WeakRef<Entity>;
}

impl AsIComponent for Ref<IComponent> {
    fn get_entity(&self) -> WeakRef<Entity> {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(IComponent::NAME)).unwrap();
        let method = cls.get_method(CName::new("GetEntity")).ok().unwrap();
        method
            .as_function()
            .execute::<_, WeakRef<Entity>>(unsafe { self.instance() }.map(AsRef::as_ref), ())
            .unwrap()
    }
}

impl AsIComponent for Ref<IPlacedComponent> {
    fn get_entity(&self) -> WeakRef<Entity> {
        unsafe { mem::transmute::<&Ref<IPlacedComponent>, &Ref<IComponent>>(self) }.get_entity()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct IPlacedComponent {
    pub base: IComponent,                                // 0x0
    pub parent_transform: DebugIgnore<Ref<IScriptable>>, // 0x90
    pub _padding2: [u8; 0x20],
    pub local_transform: WorldTransform, // 0xC0
    pub _padding3: [u8; 0x40],
}

unsafe impl ScriptClass for IPlacedComponent {
    const NAME: &'static str = "entIPlacedComponent";
    type Kind = Native;
}

impl AsRef<IComponent> for IPlacedComponent {
    #[inline]
    fn as_ref(&self) -> &IComponent {
        &self.base
    }
}

impl AsRef<IScriptable> for IPlacedComponent {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct AudioEmitterComponent {
    base: IPlacedComponent,                     // 0
    unk120: [u8; 0x138 - 0x120],                // 120
    pub update_distance: f32,                   // 138
    unk13c: [u8; 0x140 - 0x13C],                // 13C
    pub emitter_name: CName,                    // 140
    pub emitter_type: EntityEmitterContextType, // 148
    unk14c: [u8; 0x150 - 0x14C],                // 14C
    pub on_attach: AudioSyncs,                  // 150
    pub on_detach: AudioSyncs,                  // 190
    pub tags: RedArray<CName>,                  // 1D0
    pub tag_list: DebugIgnore<RedTagList>,      // 1E0
    pub emitter_metadata_name: CName,           // 1F0
    unk1f8: [u8; 0x200 - 0x1F8],                // 1F8
}

unsafe impl ScriptClass for AudioEmitterComponent {
    type Kind = Native;
    const NAME: &'static str = "gameAudioEmitterComponent";
}

impl AsRef<IScriptable> for AudioEmitterComponent {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum EntityEmitterContextType {
    #[default]
    EntityEmitter = 0,
    IndependentEmitter = 1,
    RadioEmitter = 2,
    PermanentObjectEmitter = 3,
}

#[derive(Debug)]
#[repr(C)]
pub struct AudioSyncs {
    switch_events: RedArray<AudSwitch>,       // 00
    play_events: RedArray<AudEventStruct>,    // 10
    stop_events: RedArray<AudEventStruct>,    // 20
    parameter_events: RedArray<AudParameter>, // 30
}

unsafe impl NativeRepr for AudioSyncs {
    const NAME: &'static str = "gameAudioSyncs";
}
