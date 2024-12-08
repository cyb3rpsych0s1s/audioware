//! Manifest errors.

use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    /// Binary is not located where expected.
    #[snafu(display("unable to read binary location"))]
    BinaryLocation { source: std::io::Error },
    /// Missing folder.
    #[snafu(display("unable to locate parent folder (expected '{folder}')"))]
    NoFolder { folder: &'static str },
    /// An error occured while reading a [Depot](crate::Depot) folder.
    #[snafu(display("cannot read folder: {depot}"))]
    CannotReadDepot { depot: String },
    #[snafu(
        display("cannot read file: {manifest}"),
        visibility(pub),
        context(suffix(false))
    )]
    /// An error occured while reading a [Manifest](crate::Manifest) file.
    CannotReadManifest {
        manifest: String,
        source: std::io::Error,
    },
    #[snafu(
        display("cannot parse file: {manifest}\n{source}"),
        visibility(pub),
        context(suffix(false))
    )]
    /// An error occured while parsing a [Manifest](crate::Manifest) file.
    CannotParseManifest {
        manifest: String,
        source: serde_yaml::Error,
    },
}

#[derive(Debug, Snafu, PartialEq)]
pub enum ConversionError {
    /// Cyberpunk 2077 does not support this [Locale](crate::Locale).
    #[snafu(display("invalid locale: {value}"))]
    InvalidLocale { value: String },
    /// Locale type is not supported by Locale subset.
    #[snafu(display("unsupported locale for {}: {value}", r#type))]
    UnsupportedLocale { r#type: String, value: String },
    /// Cyberpunk 2077 does not support this [PlayerGender](crate::PlayerGender).
    #[snafu(display("invalid gender: {value}"))]
    InvalidGender { value: String },
    /// Audio stream buffer size is invalid.
    #[snafu(display("invalid buffer size: {value}"))]
    InvalidBufferSize { value: String },
    /// Audio stream buffer size is missing.
    #[snafu(display("missing buffer size"))]
    MissingBufferSize,
}

#[derive(Debug, Snafu)]
#[snafu(display("invalid audio setting {which}: {why}"), visibility(pub))]
pub struct ValidationError {
    pub which: &'static str,
    pub why: &'static str,
}
