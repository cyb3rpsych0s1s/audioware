//! shortcut methods and log utils

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
    pub use ok_or_continue;
    pub use ok_or_return;
}

pub static AUDIOWARE: Lazy<CName> = Lazy::new(|| CName::new_pooled("Audioware"));

#[macro_export]
macro_rules! audioware_info {
    ($($args:expr),*) => {
        let msg = format!($($args),*);
        red4ext_rs::info!("{}", msg.as_ref());
        audioware_sys::interop::codeware::mod_log(AUDIOWARE.clone(), msg.as_ref());
    }
}

#[macro_export]
macro_rules! audioware_error {
    ($($args:expr),*) => {
        let msg = format!($($args),*);
        red4ext_rs::error!("{}", msg.as_ref());
        audioware_sys::interop::codeware::mod_log(AUDIOWARE.clone(), format!("[ERROR] {}", msg.as_ref()));
    }
}

#[macro_export]
macro_rules! audioware_warn {
    ($($args:expr),*) => {
        let msg = format!($($args),*);
        red4ext_rs::warn!("{}", msg.as_ref());
        #[cfg(debug_assertions)]
        audioware_sys::interop::codeware::mod_log(AUDIOWARE.clone(), format!("[WARN] {}", msg.as_ref()));
    }
}

#[macro_export]
macro_rules! audioware_dbg {
    ($($args:expr),*) => {
        #[cfg(debug_assertions)]
        {
            let msg = format!($($args),*);
            red4ext_rs::info!("{}", msg);
        }
    }
}
