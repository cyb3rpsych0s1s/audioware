use audioware_types::event::IScriptable;
use red4ext_rs::{
    conv::{ClassType, NativeRepr},
    macros::redscript_import,
    types::{CName, RedString, Ref},
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct LocalizationPackage;

impl ClassType for LocalizationPackage {
    type BaseClass = IScriptable;

    const NAME: &'static str = "Audioware.LocalizationPackage";
}

#[redscript_import]
impl LocalizationPackage {
    /// protected func Subtitle(key: String, valueF: String, valueM: String)
    pub fn subtitle(self: &Ref<Self>, key: RedString, value_f: RedString, value_m: RedString)
        -> ();
}
