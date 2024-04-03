use kira::{tween::Value, Volume};
use serde::{de::Visitor, Deserialize};

pub struct Settings {
    pub volume: Value<Volume>,
}

impl Deserialize for Settings {
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
            type Value = Value<Volume>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Settings")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Duration, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut volume = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Volume => {
                            if volume.is_some() {
                                return Err(de::Error::duplicate_field("volume"));
                            }
                            let raw: String = map.next_value()?;
                            let chars = raw.trim().to_lowercase().chars().peekable();
                            let len = raw.len();
                            let value: f64;
                            if chars.peek() == Some(&'x') {
                                value = chars[1..].parse()?;
                                volume = Some(Volume::Amplitude(value).into());
                            } else if chars[0..len - 2] == ['d', 'b'] {
                                value = chars[0..len - 2].parse()?;
                                volume = Some(Volume::Decibels(value).into());
                            }
                        }
                    }
                }
                let volume = volume.ok_or_else(|| de::Error::missing_field("volume"))?;
                Ok(Settings { volume })
            }
        }
        const FIELDS: &[&str] = &["volume"];
        deserializer.deserialize_struct("Settings", FIELDS, SettingsVisitor)
    }
}
