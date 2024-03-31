#![allow(dead_code)]

use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Hexadecimal(usize);

impl PartialEq<usize> for Hexadecimal {
    fn eq(&self, other: &usize) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<Hexadecimal> for usize {
    fn eq(&self, other: &Hexadecimal) -> bool {
        self.eq(&other.0)
    }
}

struct HexadecimalVisitor;
impl<'de> Visitor<'de> for HexadecimalVisitor {
    type Value = Hexadecimal;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("hexadecimal string, optionally padded with leading zeros")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Hexadecimal(
            usize::from_str_radix(v.trim_start_matches('0'), 16).map_err(Error::custom)?,
        ))
    }
}

impl<'de> Deserialize<'de> for Hexadecimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(HexadecimalVisitor)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Decimal(usize);

impl PartialEq<usize> for Decimal {
    fn eq(&self, other: &usize) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<Decimal> for usize {
    fn eq(&self, other: &Decimal) -> bool {
        self.eq(&other.0)
    }
}

struct DecimalVisitor;
impl<'de> Visitor<'de> for DecimalVisitor {
    type Value = Decimal;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("decimal string, optionally padded with leading zeros")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Decimal(
            v.trim_start_matches('0')
                .parse::<usize>()
                .map_err(Error::custom)?,
        ))
    }
}

impl<'de> Deserialize<'de> for Decimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DecimalVisitor)
    }
}

#[derive(Debug, Deserialize)]
pub struct Cyberpunk2077Addresses {
    #[serde(rename = "Linker map timestamp")]
    linker_map_timestamp: LinkerMapTimestamp,
    #[serde(rename = "Preferred load address")]
    preferred_load_address: Hexadecimal,
    #[serde(rename = "Code constant offset")]
    code_constant_offset: Hexadecimal,
    #[serde(rename = "Addresses")]
    addresses: Vec<Address>,
}

#[derive(Debug)]
pub struct LinkerMapTimestamp {
    build_number: Hexadecimal,
    timestamp: chrono::NaiveDateTime,
}

impl<'de> Deserialize<'de> for LinkerMapTimestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LinkerMapTimestampVisitor;

        impl<'de> Visitor<'de> for LinkerMapTimestampVisitor {
            type Value = LinkerMapTimestamp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("build number followed by naive date between parenthesis")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if let Some((build, date)) = v.split_once(' ') {
                    return Ok(Self::Value {
                        build_number: HexadecimalVisitor.visit_str(build)?,
                        timestamp: chrono::NaiveDateTime::parse_from_str(
                            date.trim_start_matches('(').trim_end_matches(')'),
                            // Fri Mar 15 12:56:23 2024
                            "%a %b %d %X %Y",
                        )
                        .map_err(Error::custom)?,
                    });
                }
                Err(Error::custom(format!(
                    "unknown linker map timestamp format ({})",
                    v
                )))
            }
        }

        deserializer.deserialize_str(LinkerMapTimestampVisitor)
    }
}

#[derive(Debug, Deserialize)]
pub struct Address {
    hash: Decimal,
    symbol: String,
    offset: Offset,
}

#[derive(Debug)]
pub struct Offset {
    index: Decimal,
    hex: Hexadecimal,
}

impl<'de> Deserialize<'de> for Offset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OffsetVisitor;

        impl<'de> Visitor<'de> for OffsetVisitor {
            type Value = Offset;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("index and offset, separated by colon")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some((index, hex)) = v.split_once(':') {
                    return Ok(Self::Value {
                        index: DecimalVisitor.visit_str(index)?,
                        hex: HexadecimalVisitor.visit_str(hex)?,
                    });
                }
                Err(Error::custom(format!("unknown offset format ({})", v)))
            }
        }

        deserializer.deserialize_str(OffsetVisitor)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Datelike;
    use chrono::Timelike;
    use serde_json::json;

    use crate::Address;
    use crate::Cyberpunk2077Addresses;

    use super::LinkerMapTimestamp;

    use super::Offset;

    #[test]
    pub fn linker_map_timestamp() {
        let json = "65f43767 (Fri Mar 15 12:56:23 2024)";
        let lmt: LinkerMapTimestamp = serde_json::from_value(json!(json)).unwrap();
        assert_eq!(lmt.build_number, 0x65f43767);
        assert_eq!(lmt.timestamp.weekday(), chrono::Weekday::Fri);
        assert_eq!(
            lmt.timestamp.month(),
            chrono::Month::March.number_from_month()
        );
        assert_eq!(lmt.timestamp.day(), 15);
        assert_eq!(lmt.timestamp.time().hour(), 12);
        assert_eq!(lmt.timestamp.time().minute(), 56);
        assert_eq!(lmt.timestamp.time().second(), 23);
        assert_eq!(lmt.timestamp.year(), 2024);
    }

    #[test]
    pub fn offset() {
        let json = "0001:0011fab4";
        let offset: Offset = serde_json::from_value(json!(json)).unwrap();
        assert_eq!(offset.index, 1);
        assert_eq!(offset.hex, 0x11fab4);
    }

    #[test]
    pub fn address() {
        let json = r#"{"hash": "4069332669","secondary hash": "90ccbf0f690cad932aa6cc8c6dc926eea8aaedcb0e11ee3cd3d738e7ea40ca48","symbol": "red::GameAppShutdownState::OnTick","offset": "0001:000fd288"}"#;
        let address: Address = serde_json::from_str(json).unwrap();
        assert_eq!(address.hash, 4069332669);
        assert_eq!(address.symbol, "red::GameAppShutdownState::OnTick");
        assert_eq!(address.offset.index, 1);
        assert_eq!(address.offset.hex, 0xfd288);
    }

    #[test]
    pub fn cyberpunk2077_addresses() {
        let json = r#"{"Linker map timestamp": "65f43767 (Fri Mar 15 12:56:23 2024)","Preferred load address": "0000000140000000","Code constant offset": "1000","Addresses": [{"hash": "4069332669","secondary hash": "90ccbf0f690cad932aa6cc8c6dc926eea8aaedcb0e11ee3cd3d738e7ea40ca48","symbol": "red::GameAppShutdownState::OnTick","offset": "0001:000fd288"}]}"#;
        let addresses: Cyberpunk2077Addresses = serde_json::from_str(json).unwrap();
        assert_eq!(addresses.preferred_load_address, 0x140000000);
        assert_eq!(addresses.code_constant_offset, 0x1000);
    }
}
