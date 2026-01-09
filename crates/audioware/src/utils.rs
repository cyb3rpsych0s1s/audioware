//! Logging utils.
#![allow(dead_code)]

use red4ext_rs::{
    IntoRepr, RttiSystem, log,
    types::{CName, ScriptRef},
};
use windows::{
    Win32::{Foundation::HMODULE, System::LibraryLoader::GetModuleHandleW},
    core::w,
};

use crate::Audioware;

macro_rules! silly {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            use ::red4ext_rs::PluginOps;
            ::red4ext_rs::log::info!($crate::Audioware::env(), $($arg)*)
        }
    }};
}
pub(crate) use silly;

macro_rules! lifecycle {
    ($($arg:tt)*) => {{
        #[allow(unused_variables, reason = "unused lint")]
        let msg = format!($($arg)*);
        $crate::utils::silly!("{msg}")
    }};
}
pub(crate) use lifecycle;

#[allow(unused_macros)]
macro_rules! intercept {
    ($($arg:tt)*) => {{
        #[allow(unused_variables, reason = "unused lint")]
        let msg = format!($($arg)*);
        $crate::utils::silly!("[{:?}] *~ {msg}", std::thread::current().id())
    }};
}
#[allow(unused_imports)]
pub(crate) use intercept;

#[allow(unused_macros)]
macro_rules! inspect {
    ($($arg:tt)*) => {{
        if cfg!(debug_assertions) && cfg!(feature = "research") && cfg!(feature = "redengine") {
            $crate::utils::silly!("[{:?}] *~ {}", std::thread::current().id(), format!($($arg)*))
        }
    }};
}
#[allow(unused_imports)]
pub(crate) use inspect;

macro_rules! reports {
    ([$fn:ident];[$($red4ext:tt)*];[$($reds:tt)*]) => {{
        use ::red4ext_rs::PluginOps;
        ::red4ext_rs::log::$fn!($crate::Audioware::env(), $($red4ext)*);
        $crate::utils::$fn(format!($($reds)*));
    }};
    ([error];[$($msg:tt)*]) => {{
        $crate::utils::reports!([error];[$($msg)*];[$($msg)*]);
    }};
    ([warn];[$($msg:tt)*]) => {{
        $crate::utils::reports!([warn];[$($msg)*];[$($msg)*]);
    }};
    ([info];[$($msg:tt)*]) => {{
        $crate::utils::reports!([info];[$($msg)*];[$($msg)*]);
    }};
}
pub(crate) use reports;

macro_rules! fails {
    ([$($red4ext:tt)*];[$($reds:tt)*]) => {{
        $crate::utils::reports!([error];[$($red4ext)*];[$($reds)*]);
    }};
    ($($arg:tt)*) => {{
        $crate::utils::reports!([error];[$($arg)*]);
    }};
}
pub(crate) use fails;

macro_rules! warns {
    ([$($red4ext:tt)*];[$($reds:tt)*]) => {{
        $crate::utils::reports!([warn];[$($red4ext)*];[$($reds)*]);
    }};
    ($($arg:tt)*) => {{
        $crate::utils::reports!([warn];[$($arg)*]);
    }};
}
pub(crate) use warns;

macro_rules! success {
    ([$($red4ext:tt)*];[$($reds:tt)*]) => {{
        $crate::utils::reports!([info];[$($red4ext)*];[$($reds)*]);
    }};
    ($($arg:tt)*) => {{
        $crate::utils::reports!([info];[$($arg)*]);
    }};
}
pub(crate) use success;

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
#[cfg(debug_assertions)]
pub fn dbg(msg: impl Into<String>) {
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

pub fn get_base_offset() -> HMODULE {
    match unsafe { GetModuleHandleW(w!("Cyberpunk2077.exe")) } {
        Ok(x) => x,
        Err(e) => {
            fails!("unable to determine Cyberpunk2077.exe base address: {e}");
            HMODULE(std::ptr::null_mut())
        }
    }
}
