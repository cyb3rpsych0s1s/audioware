macro_rules! ok_or_return {
    ($lock:expr, $err:literal) => {
        match $lock {
            Ok(x) => x,
            Err(e) => {
                use red4ext_rs::PluginOps;
                red4ext_rs::log::error!($crate::Audioware::env(), "{}: {}", $err, e);
                return;
            }
        }
    };
}
pub(crate) use ok_or_return;
macro_rules! some_or_return {
    ($lock:expr, $err:literal, $detail:ident) => {
        match $lock {
            Some(x) => x,
            None => {
                use red4ext_rs::PluginOps;
                red4ext_rs::log::error!($crate::Audioware::env(), "{}: {:?}", $err, $detail);
                return;
            }
        }
    };
}
pub(crate) use some_or_return;
