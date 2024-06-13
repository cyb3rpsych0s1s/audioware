use red4ext_rs::{
    macros::{redscript_global, redscript_import},
    types::{CName, RedString, Ref, ResRef, ScriptRef},
};

use super::entity::Entity;

pub mod localization;
pub mod reflection;

#[redscript_global(name = "ModLog", native)]
fn native_mod_log(r#mod: CName, text: ScriptRef<RedString>) -> ();

pub fn mod_log(r#mod: CName, text: impl AsRef<str>) {
    let mut msg = RedString::new(text);
    let msg = ScriptRef::new(&mut msg);
    native_mod_log(r#mod, msg);
}

#[redscript_import]
impl Entity {
    /// public native func GetTemplatePath() -> ResRef
    #[redscript(native)]
    pub fn get_template_path(self: &Ref<Self>) -> ResRef;
}

impl Entity {
    pub fn get_template_gender(self: &Ref<Self>) -> CName {
        let res = self.get_template_path();
        let str = red4ext_rs::call!(["ResRef"] :: ["ToString"] (res) -> RedString);
        match str.as_str() {
            x if x == "Female" || x == "Male" => CName::new(x),
            _ => CName::new("None"),
        }
    }
}
