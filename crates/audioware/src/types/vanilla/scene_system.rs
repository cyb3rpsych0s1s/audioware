use red4ext_rs::{
    RttiSystem, ScriptClass,
    class_kind::Native,
    types::{CName, EntityId, IScriptable, Method, Ref},
};

#[repr(C)]
pub struct SceneSystem {
    base: IScriptable,
    _padding0: [u8; 0x8],
}

unsafe impl ScriptClass for SceneSystem {
    const NAME: &'static str = "scnISceneSystem";
    type Kind = Native;
}

impl AsRef<IScriptable> for SceneSystem {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}

pub trait AsSceneSystem {
    fn get_script_interface(&self) -> Ref<SceneSystemInterface>;
}

impl AsSceneSystem for Ref<SceneSystem> {
    fn get_script_interface(&self) -> Ref<SceneSystemInterface> {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(SceneSystem::NAME)).unwrap();
        let method: &Method = cls
            .get_method(CName::new("GetScriptInterface"))
            .ok()
            .unwrap();
        method
            .as_function()
            .execute::<_, Ref<SceneSystemInterface>>(
                unsafe { self.instance() }.map(AsRef::as_ref),
                (),
            )
            .unwrap()
    }
}

#[repr(C)]
pub struct SceneSystemInterface {
    pub base: IScriptable,
    pub _padding0: [u8; 0x8],
}

unsafe impl ScriptClass for SceneSystemInterface {
    const NAME: &'static str = "scnScriptInterface";
    type Kind = Native;
}

impl AsRef<IScriptable> for SceneSystemInterface {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}

pub trait AsSceneSystemInterface {
    fn is_entity_in_scene(&self, entity_id: EntityId) -> bool;
}

impl AsSceneSystemInterface for Ref<SceneSystemInterface> {
    fn is_entity_in_scene(&self, entity_id: EntityId) -> bool {
        let rtti = RttiSystem::get();
        let cls = rtti
            .get_class(CName::new(SceneSystemInterface::NAME))
            .unwrap();
        let method: &Method = cls.get_method(CName::new("IsEntityInScene")).ok().unwrap();
        method
            .as_function()
            .execute::<_, bool>(unsafe { self.instance() }.map(AsRef::as_ref), (entity_id,))
            .unwrap()
    }
}
