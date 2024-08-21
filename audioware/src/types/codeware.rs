//! Interop types for [Codeware](https://github.com/psiberx/cp2077-codeware/wiki).

use red4ext_rs::{
    call,
    class_kind::{Native, Scripted},
    log,
    types::{CName, Class, EntityId, IScriptable, Ref, ResRef, TweakDbId},
    PluginOps, ScriptClass,
};

use crate::Audioware;

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
pub struct EntityTarget {
    base: CallbackSystemTarget,
    entity_id: EntityId,       // 0x40
    entity_type: *const Class, // 0x48
    record_id: TweakDbId,      // 0x50
    template_path: ResRef,     // 0x58
    appearance_name: CName,    // 0x60
}
unsafe impl ScriptClass for EntityTarget {
    type Kind = Native;
    const NAME: &'static str = "EntityTarget";
}
