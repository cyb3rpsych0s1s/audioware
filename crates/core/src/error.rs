use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(display("invalid audio setting {which}: {why}"), visibility(pub))]
pub struct ValidationError {
    pub which: &'static str,
    pub why: &'static str,
}
