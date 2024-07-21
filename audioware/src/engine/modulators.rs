use kira::{effect::EffectBuilder, manager::AudioManager, tween::Tween};

use crate::error::Error;

mod volume;
pub use volume::*;

pub trait Parameter {
    type Value;
    fn setup(manager: &mut AudioManager) -> Result<(), Error>;
    fn effect() -> Result<impl EffectBuilder, Error>;
    fn update(value: Self::Value, tween: Tween) -> Result<bool, Error>;
}
