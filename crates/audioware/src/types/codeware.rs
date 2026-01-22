//! Interop types for [Codeware](https://github.com/psiberx/cp2077-codeware/wiki).

use red4ext_rs::{
    PluginOps, RttiSystem, ScriptClass, call,
    class_kind::{Native, Scripted},
    log,
    types::{
        CName, Class, EntityId, IScriptable, NativeGameInstance, NodeRef, Ref, ResRef, TweakDbId,
    },
};

use crate::{Audioware, Entity, utils::fails};

/// Interop type for [localization package](https://github.com/psiberx/cp2077-codeware/wiki#localization-packages).
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
        }
    }
}

/// Interop type for [game events](https://github.com/psiberx/cp2077-codeware/wiki#game-events).
#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub struct CallbackSystemTarget {
    base: IScriptable,
}
unsafe impl ScriptClass for CallbackSystemTarget {
    type Kind = Native;
    const NAME: &'static str = "CallbackSystemTarget";
}

#[allow(dead_code)]
const PADDING_68: usize = 0x68 - 0x40;

/// Interop type for [game events](https://github.com/psiberx/cp2077-codeware/wiki#game-events).
#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub struct EmitterTarget {
    base: CallbackSystemTarget,
    entity_id: EntityId,       // 0x40
    entity_type: *const Class, // 0x48
    record_id: TweakDbId,      // 0x50
    template_path: ResRef,     // 0x58
    appearance_name: CName,    // 0x60
}
unsafe impl ScriptClass for EmitterTarget {
    type Kind = Native;
    const NAME: &'static str = "EmitterTarget";
}

pub trait AsNativeSystem {
    fn is_ready(&self) -> bool;
}

pub trait AsNativeSubSystem {
    fn is_ready(&self) -> bool;
}

#[derive(Debug)]
#[repr(C)]
pub struct StaticEntitySystem {
    base: IGameSystem, // 0
}
unsafe impl ScriptClass for StaticEntitySystem {
    type Kind = Native;
    const NAME: &'static str = "StaticEntitySystem";
}
impl AsRef<IScriptable> for StaticEntitySystem {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct DynamicEntitySystem {
    base: IGameSystem, // 0
}
unsafe impl ScriptClass for DynamicEntitySystem {
    type Kind = Native;
    const NAME: &'static str = "DynamicEntitySystem";
}
impl AsRef<IScriptable> for DynamicEntitySystem {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base
    }
}

pub trait AsNativeEntitySystem: AsNativeSubSystem {
    fn get_entity(&self, id: EntityId) -> Ref<Entity>;
}

impl AsNativeSubSystem for Ref<StaticEntitySystem> {
    fn is_ready(&self) -> bool {
        is_ready(self)
    }
}

impl AsNativeSubSystem for Ref<DynamicEntitySystem> {
    fn is_ready(&self) -> bool {
        is_ready(self)
    }
}

impl AsNativeEntitySystem for Ref<StaticEntitySystem> {
    fn get_entity(&self, id: EntityId) -> Ref<Entity> {
        get_entity(self, id)
    }
}

impl AsNativeEntitySystem for Ref<DynamicEntitySystem> {
    fn get_entity(&self, id: EntityId) -> Ref<Entity> {
        get_entity(self, id)
    }
}

impl AsNativeSubSystem for Ref<WorldStateSystem> {
    fn is_ready(&self) -> bool {
        is_ready(self)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct WorldStateSystem {
    base: IGameSystem, // 0
}
unsafe impl ScriptClass for WorldStateSystem {
    type Kind = Native;
    const NAME: &'static str = "WorldStateSystem";
}
impl AsRef<IScriptable> for WorldStateSystem {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base
    }
}

pub trait AsWorldStateSystem {
    fn get_population_spawner(&self, spawner: NodeRef) -> Ref<PopulationSpawnerWrapper>;
}

#[derive(Debug)]
#[repr(C)]
pub struct PopulationSpawnerWrapper {
    base: IScriptable, // 0
}
unsafe impl ScriptClass for PopulationSpawnerWrapper {
    type Kind = Native;
    const NAME: &'static str = "PopulationSpawnerWrapper";
}
impl AsRef<IScriptable> for PopulationSpawnerWrapper {
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}

pub trait AsPopulationSpawnerWrapper {
    fn is_active(&self) -> bool;
    fn is_initialized(&self) -> bool;
    fn get_active_entity_ids(&self) -> Vec<EntityId>;
}

impl AsPopulationSpawnerWrapper for Ref<PopulationSpawnerWrapper> {
    fn get_active_entity_ids(&self) -> Vec<EntityId> {
        if self.is_null() {
            return Default::default();
        }
        let rtti = RttiSystem::get();
        if let Some(x) = self.clone().cast::<IScriptable>()
            && let Some(class_name) = unsafe { x.fields() }.map(|x| x.class().name())
            && let Some(class) = rtti.get_class(class_name)
            && let Some(func) = class
                .methods()
                .iter()
                .map(|x| x.as_function())
                .find(|x| x.name() == CName::new("GetActiveEntityIDs"))
        {
            return func
                .execute::<_, Vec<EntityId>>(unsafe { x.fields() }, ())
                .unwrap_or_default();
        }
        Default::default()
    }

    fn is_active(&self) -> bool {
        if self.is_null() {
            return Default::default();
        }
        let rtti = RttiSystem::get();
        if let Some(receiver) = self.clone().cast::<IScriptable>()
            && let Some(class_name) = unsafe { receiver.fields() }.map(|x| x.class().name())
            && let Some(class) = rtti.get_class(class_name)
            && let Some(func) = class
                .methods()
                .iter()
                .map(|x| x.as_function())
                .find(|x| x.name() == CName::new("IsActive"))
        {
            return match func.execute::<_, bool>(unsafe { receiver.fields() }, ()) {
                Ok(x) => x,
                Err(e) => {
                    fails!("error calling is_active: {e}");
                    Default::default()
                }
            };
        }
        Default::default()
    }

    fn is_initialized(&self) -> bool {
        if self.is_null() {
            return Default::default();
        }
        let rtti = RttiSystem::get();
        if let Some(receiver) = self.clone().cast::<IScriptable>()
            && let Some(class_name) = unsafe { receiver.fields() }.map(|x| x.class().name())
            && let Some(class) = rtti.get_class(class_name)
            && let Some(func) = class
                .methods()
                .iter()
                .map(|x| x.as_function())
                .find(|x| x.name() == CName::new("IsInitialized"))
        {
            return match func.execute::<_, bool>(unsafe { receiver.fields() }, ()) {
                Ok(x) => x,
                Err(e) => {
                    fails!("error calling is_initialized: {e}");
                    Default::default()
                }
            };
        }
        Default::default()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct IGameSystem {
    base: IUpdatableSystem,          // 0
    game: *const NativeGameInstance, // 40
}
unsafe impl ScriptClass for IGameSystem {
    type Kind = Native;
    const NAME: &'static str = "gameIGameSystem";
}
impl AsRef<IScriptable> for IGameSystem {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct IUpdatableSystem {
    base: IScriptable, // 0
}
unsafe impl ScriptClass for IUpdatableSystem {
    type Kind = Native;
    const NAME: &'static str = "IUpdatableSystem";
}

fn is_ready<T: AsRef<IScriptable> + ScriptClass>(receiver: &Ref<T>) -> bool {
    if receiver.is_null() {
        return Default::default();
    }
    let rtti = RttiSystem::get();
    if let Some(receiver) = receiver.clone().cast::<IScriptable>()
        && let Some(class_name) = unsafe { receiver.fields() }.map(|x| x.class().name())
        && let Some(class) = rtti.get_class(class_name)
        && let Some(func) = class
            .methods()
            .iter()
            .map(|x| x.as_function())
            .find(|x| x.name() == CName::new("IsReady"))
    {
        return match func.execute::<_, bool>(unsafe { receiver.fields() }, ()) {
            Ok(x) => x,
            Err(e) => {
                fails!("error calling is_ready: {e}");
                Default::default()
            }
        };
    }
    Default::default()
}

fn get_entity<T: AsRef<IScriptable> + ScriptClass>(receiver: &Ref<T>, id: EntityId) -> Ref<Entity> {
    if receiver.is_null() {
        return Default::default();
    }
    let rtti = RttiSystem::get();
    if let Some(receiver) = receiver.clone().cast::<IScriptable>()
        && let Some(class_name) = unsafe { receiver.fields() }.map(|x| x.class().name())
        && let Some(class) = rtti.get_class(class_name)
        && let Some(func) = class
            .methods()
            .iter()
            .map(|x| x.as_function())
            .find(|x| x.name() == CName::new("GetEntity"))
    {
        return match func.execute::<_, Ref<Entity>>(unsafe { receiver.fields() }, (id,)) {
            Ok(x) => x,
            Err(e) => {
                fails!("error calling get_entity: {e}");
                Default::default()
            }
        };
    }
    Default::default()
}
