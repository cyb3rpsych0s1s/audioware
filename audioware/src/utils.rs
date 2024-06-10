//! shortcut methods and log utils

use audioware_sys::interop::codeware::mod_log;
use once_cell::sync::Lazy;
use red4ext_rs::types::CName;

pub mod macros {
    #[macro_export]
    macro_rules! ok_or_return {
        ($expression:expr, $default:expr) => {
            match $expression {
                Ok(yes) => yes,
                Err(no) => {
                    ::red4ext_rs::error!("{no}");
                    return $default;
                }
            }
        };
    }
    #[macro_export]
    macro_rules! ok_or_continue {
        ($expression:expr) => {
            match $expression {
                Ok(yes) => yes,
                Err(no) => {
                    ::red4ext_rs::error!("{no}");
                    continue;
                }
            }
        };
    }
    #[macro_export]
    macro_rules! safe_call {
        ($expression:expr) => {
            if let Err(ref e) = $expression {
                match e {
                    crate::engine::error::Error::BankRegistry { source } => match source {
                        crate::bank::error::registry::Error::MissingLocale { .. }
                        | crate::bank::error::registry::Error::RequireGender { .. } => {
                            red4ext_rs::warn!("{e}")
                        }
                        crate::bank::error::registry::Error::NotFound { .. } => {
                            red4ext_rs::error!("{e}")
                        }
                    },
                    e => red4ext_rs::error!("{e}"),
                }
            }
        };
    }
    pub use ok_or_continue;
    pub use ok_or_return;
    pub use safe_call;
}

static AUDIOWARE: Lazy<CName> = Lazy::new(|| CName::new_pooled("Audioware"));
pub fn info(msg: impl AsRef<str>) {
    red4ext_rs::info!("{}", msg.as_ref());
    mod_log(AUDIOWARE.clone(), msg.as_ref());
}
pub fn error(msg: impl AsRef<str>) {
    red4ext_rs::error!("{}", msg.as_ref());
    mod_log(AUDIOWARE.clone(), format!("[ERROR] {}", msg.as_ref()));
}
pub fn warn(msg: impl AsRef<str>) {
    red4ext_rs::warn!("{}", msg.as_ref());
    #[cfg(debug_assertions)]
    mod_log(AUDIOWARE.clone(), format!("[WARN] {}", msg.as_ref()));
}
pub fn dbg(msg: impl AsRef<str>) {
    #[cfg(debug_assertions)]
    red4ext_rs::info!("{}", msg.as_ref());
}
