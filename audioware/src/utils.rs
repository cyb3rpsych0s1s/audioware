//! Logging and general utils.

use std::sync::Mutex;

use once_cell::sync::Lazy;
use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    rngs::SmallRng,
    Rng, SeedableRng,
};
use red4ext_rs::{
    log,
    types::{CName, ScriptRef},
    IntoRepr, PluginOps, RttiSystem,
};

use crate::Audioware;

static RAND: Lazy<Mutex<SmallRng>> = Lazy::new(|| Mutex::new(SmallRng::from_entropy()));

pub fn rand<T: SampleUniform>(range: impl SampleRange<T>, default: T) -> T {
    RAND.try_lock()
        .map(|mut x| x.gen_range(range))
        .unwrap_or(default)
}

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
