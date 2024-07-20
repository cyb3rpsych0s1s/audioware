use std::sync::{OnceLock, RwLock};

use audioware_manifest::PlayerGender;
use red4ext_rs::{log, PluginOps};

use crate::{
    error::InternalError,
    types::{SpokenLocale, WrittenLocale},
    Audioware,
};

use super::State;

pub fn gender() -> &'static RwLock<PlayerGender> {
    static INSTANCE: OnceLock<RwLock<PlayerGender>> = OnceLock::new();
    INSTANCE.get_or_init(Default::default)
}

fn spoken_language() -> &'static RwLock<SpokenLocale> {
    static INSTANCE: OnceLock<RwLock<SpokenLocale>> = OnceLock::new();
    INSTANCE.get_or_init(Default::default)
}

fn written_language() -> &'static RwLock<WrittenLocale> {
    static INSTANCE: OnceLock<RwLock<WrittenLocale>> = OnceLock::new();
    INSTANCE.get_or_init(Default::default)
}

impl State for PlayerGender {
    type Value = PlayerGender;

    fn set(value: Self::Value) -> Self::Value {
        let env = Audioware::env();
        match gender().try_write() {
            Ok(mut x) => {
                let prior = *x;
                *x = value;
                log::info!(env, "gender: {prior} -> {value}");
                return prior;
            }
            Err(_) => {
                let env = Audioware::env();
                log::error!(
                    env,
                    "{}",
                    InternalError::Contention {
                        origin: "write gender"
                    }
                );
            }
        };
        Self::Value::default()
    }

    fn get() -> Self::Value {
        match gender().try_read() {
            Ok(x) => {
                return *x;
            }
            Err(_) => {
                let env = Audioware::env();
                log::error!(
                    env,
                    "{}",
                    InternalError::Contention {
                        origin: "read gender"
                    }
                );
            }
        };
        Self::Value::default()
    }
}

impl State for SpokenLocale {
    type Value = SpokenLocale;

    fn set(value: Self::Value) -> Self::Value {
        let env = Audioware::env();
        match spoken_language().try_write() {
            Ok(mut x) => {
                let prior = *x;
                *x = value;
                log::info!(env, "spoken locale: {prior} -> {value}");
                return prior;
            }
            Err(_) => {
                let env = Audioware::env();
                log::error!(
                    env,
                    "{}",
                    InternalError::Contention {
                        origin: "write spoken locale"
                    }
                );
            }
        };
        Self::Value::default()
    }

    fn get() -> Self::Value {
        match spoken_language().try_read() {
            Ok(x) => {
                return *x;
            }
            Err(_) => {
                let env = Audioware::env();
                log::error!(
                    env,
                    "{}",
                    InternalError::Contention {
                        origin: "read spoken locale"
                    }
                );
            }
        };
        Self::Value::default()
    }
}

impl State for WrittenLocale {
    type Value = WrittenLocale;

    fn set(value: Self::Value) -> Self::Value {
        let env = Audioware::env();
        match written_language().try_write() {
            Ok(mut x) => {
                let prior = *x;
                *x = value;
                log::info!(env, "written locale: {prior} -> {value}");
                return prior;
            }
            Err(_) => {
                log::error!(
                    env,
                    "{}",
                    InternalError::Contention {
                        origin: "write written locale"
                    }
                );
            }
        };
        Self::Value::default()
    }

    fn get() -> Self::Value {
        match written_language().try_read() {
            Ok(x) => {
                return *x;
            }
            Err(_) => {
                let env = Audioware::env();
                log::error!(
                    env,
                    "{}",
                    InternalError::Contention {
                        origin: "read written locale"
                    }
                );
            }
        };
        Self::Value::default()
    }
}
