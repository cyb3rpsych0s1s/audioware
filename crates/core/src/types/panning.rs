use std::ops::Deref;

use serde::Deserialize;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum PanningError {
    OutOfRange,
}

pub struct Panning(kira::Panning);

impl TryFrom<f32> for Panning {
    type Error = PanningError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value >= -1.0 && value <= 1.0 {
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
        value.into()
    }
}

impl<'de> Deserialize<'de> for Panning {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PanningVisitor;

        impl<'de> serde::de::Visitor<'de> for PanningVisitor {
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
