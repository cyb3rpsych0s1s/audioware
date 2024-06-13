pub mod macros {
    #[macro_export]
    macro_rules! safe_call {
        ($expression:expr) => {
            if let Err(ref e) = $expression {
                match e {
                    $crate::engine::error::Error::BankRegistry { source } => match source {
                        ::audioware_bank::error::registry::Error::MissingLocale { .. }
                        | ::audioware_bank::error::registry::Error::RequireGender { .. } => {
                            red4ext_rs::warn!("{e}")
                        }
                        ::audioware_bank::error::registry::Error::NotFound { .. } => {
                            red4ext_rs::error!("{e}")
                        }
                    },
                    e => red4ext_rs::error!("{e}"),
                }
            }
        };
    }
}
