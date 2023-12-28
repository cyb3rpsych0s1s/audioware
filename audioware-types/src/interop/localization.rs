use crate::interop::iscriptable::IScriptable;
use red4ext_rs::{
    conv::ClassType,
    macros::redscript_import,
    types::{CName, RedString, Ref},
};

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

/// local plugin methods
#[redscript_import]
impl LocalizationPackage {
    pub fn voice_language(self: &Ref<Self>) -> CName;
    pub fn subtitle_language(self: &Ref<Self>) -> CName;
}
