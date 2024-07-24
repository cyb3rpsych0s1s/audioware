use red4ext_rs::{
    call,
    class_kind::{Native, Scripted},
    log,
    types::{CName, EntityId, IScriptable, Opt, Ref, TweakDbId},
    PluginOps, RttiSystem, ScriptClass,
};
use std::mem;

use crate::Audioware;

pub const ENTITY_LIFECYCLE_EVENT_UNINITIALIZE: &str = "Entity/Uninitialize";

#[repr(C)]
pub struct LocalizationPackage;
unsafe impl ScriptClass for LocalizationPackage {
    type Kind = Scripted;
    const NAME: &'static str = "Audioware.LocalizationPackage";
}
pub trait Subtitle {
    fn subtitle(&self, key: &str, value_f: &str, value_m: &str);
}
impl Subtitle for Ref<LocalizationPackage> {
    /// protected func Subtitle(key: String, valueF: String, valueM: String)
    fn subtitle(&self, key: &str, value_f: &str, value_m: &str) {
        let env = Audioware::env();
        if let Err(e) = call!(self, "Subtitle;StringStringString"(key, value_f, value_m) -> ()) {
            log::error!(env, "failed to call LocalizationPackage.Subtitle: {e}");
        } else {
            log::info!(env, "subtitle executed succesfully");
        }
    }
}

#[repr(C)]
pub struct CallbackSystem;
unsafe impl ScriptClass for CallbackSystem {
    type Kind = Native;
    const NAME: &'static str = "CallbackSystem";
}

impl AsRef<IScriptable> for CallbackSystem {
    fn as_ref(&self) -> &IScriptable {
        todo!()
    }
}

pub trait AsCallbackSystem {
    /// `public native func RegisterCallback(eventName: CName, target: ref<IScriptable>, function: CName, opt sticky: Bool) -> ref<CallbackSystemHandler>`
    fn register_callback(
        &self,
        event_name: CName,
        target: Ref<IScriptable>,
        function: CName,
        sticky: Opt<bool>,
    ) -> Ref<CallbackSystemHandler>;
    /// `public native func UnregisterCallback(eventName: CName, target: ref<IScriptable>, opt function: CName)`
    fn unregister_callback(
        &self,
        event_name: CName,
        target: Ref<IScriptable>,
        function: Opt<CName>,
    );
}

impl AsCallbackSystem for Ref<CallbackSystem> {
    fn register_callback(
        &self,
        event_name: CName,
        target: Ref<IScriptable>,
        function: CName,
        sticky: Opt<bool>,
    ) -> Ref<CallbackSystemHandler> {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(CallbackSystem::NAME)).unwrap();
        let method = cls.get_method(CName::new("RegisterCallback")).ok().unwrap();
        let me: Ref<IScriptable> = unsafe { mem::transmute(self.clone()) };
        method
            .as_function()
            .execute::<_, Ref<CallbackSystemHandler>>(
                unsafe { me.instance() },
                (event_name, target, function, sticky.unwrap_or_default()),
            )
            .unwrap()
    }

    fn unregister_callback(
        &self,
        event_name: CName,
        target: Ref<IScriptable>,
        function: Opt<CName>,
    ) {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(CallbackSystem::NAME)).unwrap();
        let method = cls
            .get_method(CName::new("UnregisterCallback"))
            .ok()
            .unwrap();
        let me: Ref<IScriptable> = unsafe { mem::transmute(self.clone()) };
        method
            .as_function()
            .execute::<_, ()>(
                unsafe { me.instance() },
                (event_name, target, function.unwrap_or_default()),
            )
            .unwrap()
    }
}

#[repr(C)]
pub struct CallbackSystemHandler;
unsafe impl ScriptClass for CallbackSystemHandler {
    type Kind = Native;
    const NAME: &'static str = "CallbackSystemHandler";
}

pub trait AsCallbackSystemHandler {
    /// `public native func AddTarget(target: ref<CallbackSystemTarget>) -> ref<CallbackSystemHandler>`
    fn add_target(&self, target: Ref<IScriptable>) -> Ref<CallbackSystemHandler>;
    /// `public native func RemoveTarget(target: ref<CallbackSystemTarget>) -> ref<CallbackSystemHandler>`
    fn remove_target(&self, target: Ref<IScriptable>) -> Ref<CallbackSystemHandler>;
}

impl AsCallbackSystemHandler for Ref<CallbackSystemHandler> {
    fn add_target(&self, target: Ref<IScriptable>) -> Ref<CallbackSystemHandler> {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(CallbackSystem::NAME)).unwrap();
        let method = cls.get_method(CName::new("AddTarget")).ok().unwrap();
        let me: Ref<IScriptable> = unsafe { mem::transmute(self.clone()) };
        method
            .as_function()
            .execute::<_, Ref<CallbackSystemHandler>>(unsafe { me.instance() }, (target,))
            .unwrap()
    }
    fn remove_target(&self, target: Ref<IScriptable>) -> Ref<CallbackSystemHandler> {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(CallbackSystem::NAME)).unwrap();
        let method = cls.get_method(CName::new("RemoveTarget")).ok().unwrap();
        let me: Ref<IScriptable> = unsafe { mem::transmute(self.clone()) };
        method
            .as_function()
            .execute::<_, Ref<CallbackSystemHandler>>(unsafe { me.instance() }, (target,))
            .unwrap()
    }
}

#[repr(C)]
pub struct EntityTarget;
unsafe impl ScriptClass for EntityTarget {
    type Kind = Native;
    const NAME: &'static str = "EntityTarget";
}

pub trait AsEntityTarget {
    /// `public static native func ID(entityID: EntityID) -> ref<EntityTarget>`
    fn id(entity_id: EntityId) -> Ref<EntityTarget>;
    /// `public static native func Type(entityType: CName) -> ref<EntityTarget>`
    fn ty(entity_type: CName) -> Ref<EntityTarget>;
    /// `public static native func RecordID(recordID: TweakDBID) -> ref<EntityTarget>`
    fn record_id(record_id: TweakDbId) -> Ref<EntityTarget>;
    // /// `public static native func Template(templatePath: ResRef) -> ref<EntityTarget>`
    // fn template(template_path: ResRef) -> Ref<EntityTarget>;
    /// `public static native func Appearance(appearanceName: CName) -> ref<EntityTarget>`
    fn appearance(appearance_name: CName) -> Ref<EntityTarget>;
}

impl AsEntityTarget for EntityTarget {
    fn id(entity_id: EntityId) -> Ref<EntityTarget> {
        let rtti = RttiSystem::get();
        let methods = rtti.get_global_functions();
        let method = methods
            .iter()
            .find(|x| x.name() == CName::new("ID;EntityID"))
            .unwrap();
        method
            .execute::<_, Ref<EntityTarget>>(None, (entity_id,))
            .unwrap()
    }

    fn ty(entity_type: CName) -> Ref<EntityTarget> {
        let rtti = RttiSystem::get();
        let methods = rtti.get_global_functions();
        let method = methods
            .iter()
            .find(|x| x.name() == CName::new("Type;CName"))
            .unwrap();
        method
            .execute::<_, Ref<EntityTarget>>(None, (entity_type,))
            .unwrap()
    }

    fn record_id(record_id: TweakDbId) -> Ref<EntityTarget> {
        let rtti = RttiSystem::get();
        let methods = rtti.get_global_functions();
        let method = methods
            .iter()
            .find(|x| x.name() == CName::new("RecordID;TweakDBID"))
            .unwrap();
        method
            .execute::<_, Ref<EntityTarget>>(None, (record_id,))
            .unwrap()
    }

    // fn template(template_path: ResRef) -> Ref<EntityTarget> {
    //     let rtti = RttiSystem::get();
    //     let methods = rtti.get_global_functions();
    //     let method = methods
    //         .iter()
    //         .find(|x| x.name() == CName::new("Template;ResRef"))
    //         .unwrap();
    //     method
    //         .execute::<_, Ref<EntityTarget>>(None, (template_path,))
    //         .unwrap()
    // }

    fn appearance(appearance_name: CName) -> Ref<EntityTarget> {
        let rtti = RttiSystem::get();
        let methods = rtti.get_global_functions();
        let method = methods
            .iter()
            .find(|x| x.name() == CName::new("Appearance;CName"))
            .unwrap();
        method
            .execute::<_, Ref<EntityTarget>>(None, (appearance_name,))
            .unwrap()
    }
}
