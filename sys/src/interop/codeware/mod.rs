use red4ext_rs::{
    macros::redscript_global,
    types::{CName, RedString, ScriptRef},
};

pub mod localization;
pub mod reflection;

#[redscript_global(name = "ModLog", native)]
fn native_mod_log(r#mod: CName, text: ScriptRef<RedString>) -> ();

pub fn mod_log(r#mod: CName, text: impl AsRef<str>) {
    let mut msg = RedString::new(text);
    let msg = ScriptRef::new(&mut msg);
    native_mod_log(r#mod, msg);
}
