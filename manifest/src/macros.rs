#[macro_export]
macro_rules! ok_or_return {
    ($expression:expr, $default:expr) => {
        match $expression {
            Ok(yes) => yes,
            Err(no) => {
                ::red4ext_rs::log::error!("{no}");
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
                ::red4ext_rs::log::error!("{no}");
                continue;
            }
        }
    };
}
