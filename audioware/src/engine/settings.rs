use kira::{sound::static_sound::StaticSoundSettings, tween::Value, Volume};
use serde::{
    de::{MapAccess, Visitor},
    Deserialize,
};

pub trait Safety {
    const MAX_DECIBELS: f64;
}

impl Safety for Volume {
    const MAX_DECIBELS: f64 = 70.;
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub volume: Value<Volume>,
}

impl From<Settings> for StaticSoundSettings {
    fn from(value: Settings) -> Self {
        Self::default().volume(value.volume)
    }
}

impl<'de> Deserialize<'de> for Settings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Volume,
        }
        struct SettingsVisitor;
        impl<'de> Visitor<'de> for SettingsVisitor {
            type Value = Settings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Settings")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut volume = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Volume => {
                            if volume.is_some() {
                                return Err(serde::de::Error::duplicate_field("volume"));
                            }
                            let raw: String = map.next_value()?;
                            let chars = raw.trim().to_lowercase().chars().collect::<Vec<_>>();
                            let len = raw.len();
                            let mut value: f64;
                            if chars.first() == Some(&'x') {
                                value = chars[1..]
                                    .iter()
                                    .collect::<String>()
                                    .parse::<f64>()
                                    .map_err(|_| {
                                        serde::de::Error::invalid_value(
                                            serde::de::Unexpected::Str(&raw),
                                            &self,
                                        )
                                    })?;
                                if Volume::Amplitude(value).as_decibels() < Volume::MIN_DECIBELS {
                                    red4ext_rs::warn!("volume is too low, clamped to min ({raw})");
                                    value = Volume::Decibels(Volume::MIN_DECIBELS).as_amplitude();
                                } else if Volume::Amplitude(value).as_decibels()
                                    > Volume::MAX_DECIBELS
                                {
                                    red4ext_rs::warn!("volume is too high, clamped to max ({raw})");
                                    value = Volume::Decibels(Volume::MAX_DECIBELS).as_amplitude();
                                }
                                volume = Some(Volume::Amplitude(value).into());
                            } else if chars[0..len - 2] == ['d', 'b'] {
                                value = chars[0..len - 2]
                                    .iter()
                                    .collect::<String>()
                                    .parse()
                                    .map_err(|_| {
                                        serde::de::Error::invalid_value(
                                            serde::de::Unexpected::Str(&raw),
                                            &self,
                                        )
                                    })?;
                                if value < Volume::MIN_DECIBELS {
                                    red4ext_rs::warn!("volume is too low, clamped to min ({raw})");
                                    value = Volume::MIN_DECIBELS;
                                } else if value > Volume::MAX_DECIBELS {
                                    red4ext_rs::warn!("volume is too high, clamped to max ({raw})");
                                    value = Volume::MAX_DECIBELS;
                                }
                                volume = Some(Volume::Decibels(value).into());
                            }
                        }
                    }
                }
                let volume = volume.ok_or_else(|| serde::de::Error::missing_field("volume"))?;
                Ok(Settings { volume })
            }
        }
        const FIELDS: &[&str] = &["volume"];
        deserializer.deserialize_struct("Settings", FIELDS, SettingsVisitor)
    }
}
