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
