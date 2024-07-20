use red4ext_rs::{
    log,
    types::{CName, ScriptRef},
    IntoRepr, PluginOps, RttiSystem,
};

use crate::Audioware;

pub(super) fn info(msg: impl Into<String>) {
    console(msg, "FTLog");
}

pub(super) fn warn(msg: impl Into<String>) {
    console(msg, "FTLogWarning");
}

pub(super) fn error(msg: impl Into<String>) {
    console(msg, "FTLogError");
}

#[inline]
fn console(msg: impl Into<String>, func_name: &str) {
    let env = Audioware::env();
    if let Some(ft_log) = RttiSystem::get()
        .get_global_functions()
        .iter()
        .find(|x| x.name() == CName::new(func_name))
    {
        if let Some(msg) = ScriptRef::new(&mut msg.into().into_repr()) {
            if let Err(e) = ft_log.execute::<_, ()>(None, (msg,)) {
                log::error!(env, "Failed to invoke {func_name}: {e}");
            }
        } else {
            log::error!(
                env,
                "Failed to prepare {func_name}: unable to get ScriptRef"
            );
        }
    } else {
        log::error!(env, "Redscript RTTI hasn't loaded yet")
    }
}
