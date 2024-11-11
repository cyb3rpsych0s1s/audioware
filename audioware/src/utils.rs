//! Logging utils.

use red4ext_rs::{
    log,
    types::{CName, ScriptRef},
    IntoRepr, RttiSystem,
};

use crate::Audioware;

#[allow(unused_macros)]
macro_rules! silly {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            use ::red4ext_rs::PluginOps;
            ::red4ext_rs::log::info!($crate::Audioware::env(), $($arg)*)
        }
    }};
}
#[allow(unused_imports)]
pub(crate) use silly;

#[allow(unused_macros)]
macro_rules! lifecycle {
    ($($arg:tt)*) => {
        $crate::utils::silly!($($arg)*)
    };
}
#[allow(unused_imports)]
pub(crate) use lifecycle;

#[allow(unused_macros)]
macro_rules! fails {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            use ::red4ext_rs::PluginOps;
            ::red4ext_rs::log::error!($crate::Audioware::env(), $($arg)*)
        }
    }};
}
#[allow(unused_imports)]
pub(crate) use fails;

/// Exposes `PLog` to Redscript.
pub fn plog_info(msg: String) {
    plog(msg, "PLog");
}

/// Exposes `PLogWarning` to Redscript.
pub fn plog_warn(msg: String) {
    plog(msg, "PLogWarning");
}

/// Exposes `PLogError` to Redscript.
pub fn plog_error(msg: String) {
    plog(msg, "PLogError");
}

#[inline]
fn plog(msg: String, func_name: &str) {
    use ::red4ext_rs::PluginOps;
    match func_name {
        "PLog" => {
            log::info!(Audioware::env(), "{msg}");
        }
        "PLogWarning" => {
            log::warn!(Audioware::env(), "{msg}");
        }
        "PLogError" => {
            log::error!(Audioware::env(), "{msg}");
        }
        _ => unreachable!(),
    }
}

/// Exposes `FTLog` to Rust on debug builds only.
pub fn dbg(msg: impl Into<String>) {
    #[cfg(debug_assertions)]
    console(msg, "FTLog");
}

/// Exposes `FTLog` to Rust.
pub fn info(msg: impl Into<String>) {
    console(msg, "FTLog");
}

/// Exposes `FTLogWarning` to Rust.
pub fn warn(msg: impl Into<String>) {
    console(msg, "FTLogWarning");
}

/// Exposes `FTLogError` to Rust.
pub fn error(msg: impl Into<String>) {
    console(msg, "FTLogError");
}

#[inline]
fn console(msg: impl Into<String>, func_name: &str) {
    use ::red4ext_rs::PluginOps;
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
