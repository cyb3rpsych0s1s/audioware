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
pub struct Settings<Extra: Sized> {
    pub volume: Value<Volume>,
    pub extra: Extra,
}

#[derive(Debug, Clone)]
pub struct StaticOnlySettings {
    pub reverse: bool,
}

impl From<Settings<StaticOnlySettings>> for StaticSoundSettings {
    fn from(value: Settings<StaticOnlySettings>) -> Self {
        Self::default()
            .volume(value.volume)
            .reverse(value.extra.reverse)
    }
}

impl<'de> Deserialize<'de> for Settings<StaticOnlySettings> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Volume,
            Reverse,
        }
        struct SettingsVisitor;
        impl<'de> Visitor<'de> for SettingsVisitor {
            type Value = Settings<StaticOnlySettings>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Settings")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut volume = None;
                let mut reverse = None;
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
                        Field::Reverse => {
                            if reverse.is_some() {
                                return Err(serde::de::Error::duplicate_field("reverse"));
                            }
                            reverse = Some(map.next_value()?);
                        }
                    }
                }
                if volume.is_none() && reverse.is_none() {
                    return Err(serde::de::Error::missing_field(
                        "missing at least one field: volume, reverse",
                    ));
                }
                Ok(Settings {
                    volume: volume.unwrap_or(Value::Fixed(Volume::default())),
                    extra: StaticOnlySettings {
                        reverse: reverse.unwrap_or_default(),
                    },
                })
            }
        }
        const FIELDS: &[&str] = &["volume", "reverse"];
        deserializer.deserialize_struct("Settings", FIELDS, SettingsVisitor)
    }
}
