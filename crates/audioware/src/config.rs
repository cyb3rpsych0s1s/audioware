//! Plugin config settings.

use std::path::PathBuf;

use audioware_manifest::{error::ConversionError, try_get_folder};
use ini::Ini;
use red4ext_rs::{PluginOps, log};

use crate::Audioware;

/// Engine audio backend buffer size.
///
/// See `cpal` [BufferSize](https://docs.rs/cpal/latest/cpal/enum.BufferSize.html).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(i64)]
pub enum BufferSize {
    #[default]
    Auto = 0,
    Option64 = 64,
    Option128 = 128,
    Option256 = 256,
    Option512 = 512,
    Option1024 = 1024,
    Option2048 = 2048,
}

impl BufferSize {
    /// Read ModSettings .ini file.
    pub fn read_ini() -> BufferSize {
        if let Ok(ini_filepath) = try_get_ini()
            && let Ok(conf) = Ini::load_from_file(ini_filepath)
        {
            match conf.try_into() {
                Ok(x) => return x,
                Err(ConversionError::InvalidBufferSize { value }) => {
                    log::error!(
                        Audioware::env(),
                        "Error reading ModSettings .ini: {}",
                        ConversionError::InvalidBufferSize { value }
                    );
                }
                _ => {}
            };
        }
        BufferSize::Auto
    }
}

impl TryFrom<Ini> for BufferSize {
    type Error = ConversionError;

    fn try_from(conf: Ini) -> Result<Self, Self::Error> {
        // section and value must match Redscript config naming
        if let Some(section) = conf.section(Some("Audioware.AudiowareConfig"))
            && let Some(value) = section.get("bufferSize")
        {
            match value {
                "Auto" => return Ok(Self::Auto),
                "Option64" => return Ok(Self::Option64),
                "Option128" => return Ok(Self::Option128),
                "Option256" => return Ok(Self::Option256),
                "Option512" => return Ok(Self::Option512),
                "Option1024" => return Ok(Self::Option1024),
                "Option2048" => return Ok(Self::Option2048),
                _ => {
                    return Err(ConversionError::InvalidBufferSize {
                        value: value.to_string(),
                    });
                }
            }
        }
        Err(ConversionError::MissingBufferSize)
    }
}

/// Retrieve ModSettings .ini file from filesystem.
fn try_get_ini() -> Result<PathBuf, audioware_manifest::error::Error> {
    try_get_folder(
        PathBuf::from("red4ext")
            .join("plugins")
            .join("mod_settings")
            .join("user.ini"),
    )
}
