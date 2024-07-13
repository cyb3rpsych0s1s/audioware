use red4ext_rs::{
    call, log,
    types::{Ref, ScriptClass, Scripted},
    PluginOps,
};

use crate::Audioware;

#[repr(C)]
pub struct LocalizationPackage;
unsafe impl ScriptClass for LocalizationPackage {
    type Kind = Scripted;
    const CLASS_NAME: &'static str = "Audioware.LocalizationPackage";
}
pub trait Subtitle {
    fn subtitle(self, key: &str, value_f: &str, value_m: &str);
}
impl Subtitle for Ref<LocalizationPackage> {
    /// protected func Subtitle(key: String, valueF: String, valueM: String)
    fn subtitle(self, key: &str, value_f: &str, value_m: &str) {
        if let Err(e) =
            call!(self, "Subtitle;StringStringString"(key.to_string(), value_f.to_string(), value_m.to_string()) -> ())
        {
            let env = Audioware::env();
            log::error!(env, "failed to call LocalizationPackage.Subtitle: {e}");
        }
    }
}
