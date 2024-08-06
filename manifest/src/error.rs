use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("unable to read binary location"), visibility(pub))]
    BinaryLocation { source: std::io::Error },
    #[snafu(
        display("unable to locate parent folder (expected '{folder}')"),
        visibility(pub)
    )]
    NoFolder { folder: &'static str },
    #[snafu(display("cannot read folder: {depot}"), visibility(pub))]
    CannotReadDepot { depot: String },
    #[snafu(display("cannot read file: {manifest}"), visibility(pub))]
    CannotReadManifest {
        manifest: String,
        source: std::io::Error,
    },
    #[snafu(display("cannot parse file: {manifest}\n{source}"), visibility(pub))]
    CannotParseManifest {
        manifest: String,
        source: serde_yaml::Error,
    },
}

#[derive(Debug, Snafu, PartialEq)]
pub enum ConversionError {
    #[snafu(display("invalid locale: {value}"), visibility(pub))]
    InvalidLocale { value: String },
    #[snafu(display("invalid gender: {value}"), visibility(pub))]
    InvalidGender { value: String },
    #[snafu(display("invalid buffer size: {value}"), visibility(pub))]
    InvalidBufferSize { value: String },
    #[snafu(display("missing buffer size"), visibility(pub))]
    MissingBufferSize,
}
