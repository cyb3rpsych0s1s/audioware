use red4ext_rs::{log, types::CName, RttiSystem};

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

pub fn find_class(cls_name: &str) {
    use red4ext_rs::PluginOps;
    let env = Audioware::env();
    let rtti = RttiSystem::get();
    let cls = rtti.get_class(CName::new(cls_name)).unwrap();
    log::info!(
        env,
        "{} => native: {}, size: {:#02X}, value holder size: {:#02X}, align: {:#02X}, parent: {}",
        cls.name(),
        cls.flags().is_native(),
        cls.size(),
        cls.holder_size(),
        cls.alignment(),
        cls.base().map(|x| x.name()).unwrap_or_default()
    );
    let methods = cls.methods();
    for method in methods.iter() {
        log::info!(
            env,
            "[self]    {} => {}",
            method.as_function().name(),
            method.as_function().short_name(),
        );
    }
    let static_methods = cls.static_methods();
    for static_method in static_methods.iter() {
        log::info!(
            env,
            "[static]  {} => {}",
            static_method.as_function().name(),
            static_method.as_function().short_name(),
        );
    }
}
