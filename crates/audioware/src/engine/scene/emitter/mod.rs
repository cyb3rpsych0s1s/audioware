#[allow(clippy::module_inception)]
mod emitter;
mod emitters;

pub use emitter::Emitter;
pub use emitters::{EmitterKey, Emitters, EMITTERS};
