use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum InternalError {
    #[snafu(display("{origin} contention"))]
    Contention { origin: &'static str },
}
