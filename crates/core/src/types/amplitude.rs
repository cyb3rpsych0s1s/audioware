use std::ops::Div;

use kira::Decibels;
use serde::Deserialize;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum AmplitudeError {
    #[snafu(display("amplitude must be greater or equal to 0.0"))]
    CannotBeNegative,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Amplitude(f32);

#[macro_export]
macro_rules! amplitude {
    ($v:expr) => {{
        Amplitude::try_from($v).unwrap()
    }};
}

impl Amplitude {
    pub fn as_decibels(&self) -> Decibels {
        match self.0 {
            1.0 => Decibels::IDENTITY,
            x if x < 0.0 => unreachable!(),
            x => x.into(),
        }
    }
}

impl TryFrom<f32> for Amplitude {
    type Error = AmplitudeError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value >= 0.0 {
            Ok(Amplitude(value))
        } else {
            Err(AmplitudeError::CannotBeNegative)
        }
    }
}

impl Div<f32> for Amplitude {
    type Output = f32;

    fn div(self, rhs: f32) -> Self::Output {
        self.0 / rhs
    }
}

impl From<Decibels> for Amplitude {
    fn from(value: Decibels) -> Self {
        Self(value.as_amplitude())
    }
}

impl From<Amplitude> for Decibels {
    fn from(value: Amplitude) -> Self {
        Self(20. * value.0.log10())
    }
}

impl<'de> Deserialize<'de> for Amplitude {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AmplitudeVisitor;

        impl serde::de::Visitor<'_> for AmplitudeVisitor {
            type Value = Amplitude;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a number greater or equal to 0.0")
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Amplitude::try_from(value as f32).map_err(E::custom)
            }
        }

        deserializer.deserialize_any(AmplitudeVisitor)
    }
}

impl std::fmt::Display for Amplitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
