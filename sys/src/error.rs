use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum ConversionError {
    #[snafu(display("invalid locale: {value}"))]
    InvalidLocale { value: String },
}

#[derive(Debug, Snafu)]
pub enum UpcastError {
    #[snafu(display("IScriptable has no parent class"))]
    NoParentClass,
}

#[derive(Debug, Snafu)]
pub enum DowncastError {
    #[snafu(display("unable to cast {current} to {class}"))]
    Invalid {
        current: &'static str,
        class: &'static str,
    },
}

#[derive(Debug, Snafu)]
pub enum ReflectionError {
    #[snafu(
        display("unable to retrieve class definition: {name}"),
        visibility(pub(crate))
    )]
    UnknownClass { name: &'static str },
    #[snafu(
        display("unable to retrieve func definition: {owner}.{name}"),
        visibility(pub(crate))
    )]
    UnknownFunc {
        name: &'static str,
        owner: &'static str,
    },
    #[snafu(
        display("unable to retrieve static func definition: {owner}::{name}"),
        visibility(pub(crate))
    )]
    UnknownStaticFunc {
        name: &'static str,
        owner: &'static str,
    },
    #[snafu(display("unable to cast Variant to: {name}"), visibility(pub(crate)))]
    FromVariant { name: &'static str },
}
