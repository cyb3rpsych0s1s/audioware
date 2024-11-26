use red4ext_rs::{
    class_kind::Native,
    types::{CName, Cruid, IScriptable, Ref, WeakRef},
    RttiSystem, ScriptClass,
};
use std::mem;

use super::{Entity, WorldTransform};

#[repr(C)]
pub struct IPlacedComponent {
    pub base: IComponent,                   // 0x0
    pub parent_transform: Ref<IScriptable>, // 0x90
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
