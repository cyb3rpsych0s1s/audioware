pub mod macros {
    #[macro_export]
    macro_rules! safe_call {
        ($expression:expr) => {
            if let Err(ref e) = $expression {
                match e {
                    ::audioware_engine::Error::BankRegistry { source } => match source {
                        ::audioware_bank::error::registry::Error::MissingLocale { .. }
                        | ::audioware_bank::error::registry::Error::RequireGender { .. } => {
                            ::audioware_core::audioware_warn!("{e}")
                        }
                        ::audioware_bank::error::registry::Error::NotFound { .. } => {
                            ::audioware_core::audioware_error!("{e}")
                        }
                    },
                    e => ::audioware_core::audioware_error!("{e}"),
                }
            }
        };
    }
}
