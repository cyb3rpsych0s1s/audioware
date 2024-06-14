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
