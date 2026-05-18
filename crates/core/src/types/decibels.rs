#[derive(Debug, Clone, Copy)]
pub struct Decibels(kira::Decibels);

impl Decibels {
    pub fn as_f64(&self) -> f64 {
        self.0.0 as f64
    }
}

impl From<f32> for self::Decibels {
    fn from(value: f32) -> Self {
        Self(kira::Decibels(value))
    }
}

impl From<self::Decibels> for kira::Decibels {
    fn from(value: self::Decibels) -> Self {
        value.0
    }
}

impl From<self::Decibels> for kira::Value<kira::Decibels> {
    fn from(val: self::Decibels) -> Self {
        val.0.0.into()
    }
}

impl std::fmt::Display for Decibels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}dB",
            if self.0.0.is_sign_negative() { "" } else { "+" },
            self.0.0
        )
    }
}
