use std::sync::{OnceLock, RwLock};

use audioware_manifest::{PlayerGender, SpokenLocale, WrittenLocale};
use red4ext_rs::{log, PluginOps};

use crate::{error::InternalError, Audioware};

use super::State;

pub fn gender() -> &'static RwLock<Option<PlayerGender>> {
    static INSTANCE: OnceLock<RwLock<Option<PlayerGender>>> = OnceLock::new();
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
    type Value = Option<PlayerGender>;

    fn swap(value: Self::Value) -> Self::Value {
        match gender().try_write() {
            Ok(mut x) => {
                let prior = *x;
                *x = value;
                crate::utils::silly!(
                    "gender: {} -> {}",
                    prior.map(|x| x.to_string()).unwrap_or("None".to_string()),
                    value.map(|x| x.to_string()).unwrap_or("None".to_string())
                );
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

    fn swap(value: Self::Value) -> Self::Value {
        match spoken_language().try_write() {
            Ok(mut x) => {
                let prior = *x;
                *x = value;
                crate::utils::silly!("spoken locale: {prior} -> {value}");
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

    fn swap(value: Self::Value) -> Self::Value {
        let env = Audioware::env();
        match written_language().try_write() {
            Ok(mut x) => {
                let prior = *x;
                *x = value;
                crate::utils::silly!("written locale: {prior} -> {value}");
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
