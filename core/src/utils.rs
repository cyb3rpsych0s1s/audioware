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
    ($($args:expr),*) => {{
        let msg = format!($($args),*);
        red4ext_rs::info!("{}", AsRef::<str>::as_ref(&msg));
        audioware_sys::interop::codeware::mod_log($crate::AUDIOWARE.clone(), AsRef::<str>::as_ref(&msg));
    }}
}

#[macro_export]
macro_rules! audioware_error {
    ($($args:expr),*) => {{
        let msg = format!($($args),*);
        red4ext_rs::error!("{}", AsRef::<str>::as_ref(&msg));
        audioware_sys::interop::codeware::mod_log($crate::AUDIOWARE.clone(), format!("[ERROR] {}", AsRef::<str>::as_ref(&msg)));
    }}
}

#[macro_export]
macro_rules! audioware_warn {
    ($($args:expr),*) => {{
        let msg = format!($($args),*);
        red4ext_rs::warn!("{}", AsRef::<str>::as_ref(&msg));
        #[cfg(debug_assertions)]
        audioware_sys::interop::codeware::mod_log($crate::AUDIOWARE.clone(), format!("[WARN] {}", AsRef::<str>::as_ref(&msg)));
    }}
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
