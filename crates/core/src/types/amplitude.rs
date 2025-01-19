use serde::Deserialize;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum AmplitudeError {
    #[snafu(display("amplitude must be greater or equal to 0.0"))]
    CannotBeNegative,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Amplitude(pub f32);

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

impl From<kira::Decibels> for Amplitude {    
    fn from(value: kira::Decibels) -> Self {
        Self(value.as_amplitude())
    }
}

impl From<Amplitude> for kira::Decibels {
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

        impl<'de> serde::de::Visitor<'de> for AmplitudeVisitor {
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