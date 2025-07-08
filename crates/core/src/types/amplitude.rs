use std::ops::Div;

use kira::Decibels;
use serde::Deserialize;
use snafu::Snafu;

#[inline]
fn amplitude_to_decibels(amplitude: f32) -> Decibels {
    Decibels(20. * amplitude.log10())
}

#[derive(Debug, Snafu)]
pub enum AmplitudeError {
    #[snafu(display("amplitude must be greater or equal to 0.0"))]
    CannotBeNegative,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Amplitude(f32);

#[macro_export]
macro_rules! amplitude {
    ($v:expr) => {{ Amplitude::try_from($v).unwrap() }};
}

impl Amplitude {
    pub fn as_decibels(&self) -> Decibels {
        match self.0 {
            1.0 => Decibels::IDENTITY,
            0.0 => Decibels::SILENCE,
            x if x < 0.0 => unreachable!(),
            x => amplitude_to_decibels(x),
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
        amplitude_to_decibels(value.0)
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

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
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

#[cfg(test)]
mod tests {
    use kira::Decibels;
    use test_case::test_case;

    use crate::Amplitude;

    #[test_case(0.0, Decibels::SILENCE ; "silence")]
    #[test_case(0.25118864, Decibels(-12.) ; "minus 12dB")]
    #[test_case(0.70794576, Decibels(-3.) ; "minus 3dB")]
    #[test_case(1.0, Decibels::IDENTITY ; "identity")]
    #[test_case(1.4125376, Decibels(3.0) ; "plus 3dB")]
    #[test_case(3.9810717, Decibels(12.0) ; "plus 12dB")]
    fn amplitude_as_decibels(given: f32, expected: Decibels) {
        assert_eq!(amplitude!(given).as_decibels(), expected);
    }
}
