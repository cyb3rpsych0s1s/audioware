use red4ext_rs::{log, RttiSystem};

use crate::Audioware;

pub fn list_globals() {
    use red4ext_rs::PluginOps;
    let env = Audioware::env();
    let rtti = RttiSystem::get();
    let global_methods = rtti.get_global_functions();
    for g in global_methods.iter() {
        log::info!(env, "global => {} ({})", g.name(), g.short_name());
    }
}
pub fn find_global(func_name: &str) {
    use red4ext_rs::PluginOps;
    let env = Audioware::env();
    let rtti = RttiSystem::get();
    let global_methods = rtti.get_global_functions();
    for g in global_methods.iter() {
        if g.name().as_str().contains(func_name) || g.short_name().as_str().contains(func_name) {
            log::info!(env, "global => {} ({})", g.name(), g.short_name());
        }
    }
}
