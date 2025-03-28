use std::ops::Deref;

use serde::Deserialize;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum PanningError {
    #[snafu(display("panning must be between -1.0 and 1.0 (inclusive)"))]
    OutOfRange,
}

pub struct Panning(kira::Panning);

impl TryFrom<f32> for Panning {
    type Error = PanningError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if (-1.0..=1.0).contains(&value) {
            Ok(Panning(kira::Panning(value)))
        } else {
            Err(PanningError::OutOfRange)
        }
    }
}

impl Deref for Panning {
    type Target = kira::Panning;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&Panning> for kira::Value<kira::Panning> {
    fn from(value: &Panning) -> Self {
        value.0.into()
    }
}

impl<'de> Deserialize<'de> for Panning {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PanningVisitor;

        impl serde::de::Visitor<'_> for PanningVisitor {
            type Value = Panning;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a number between -1.0 and 1.0 (inclusive)")
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Panning::try_from(value as f32).map_err(E::custom)
            }
        }

        deserializer.deserialize_any(PanningVisitor)
    }
}
