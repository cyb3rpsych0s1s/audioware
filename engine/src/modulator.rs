mod volume;

use std::sync::{Mutex, MutexGuard};

use crate::Error;
use kira::{effect::EffectBuilder, manager::AudioManager, tween::Tween};
use once_cell::sync::OnceCell;
use red4ext_rs::types::CName;
pub use volume::VolumeModulator;

static MODULATORS: OnceCell<Mutex<Vec<CName>>> = OnceCell::new();

pub struct GlobalParameters;
impl GlobalParameters {
    fn try_lock<'a>() -> Result<MutexGuard<'a, Vec<CName>>, Error> {
        Ok(MODULATORS.get_or_init(Default::default).try_lock()?)
    }
    fn register(name: &CName) -> Result<(), Error> {
        let mut modulators = Self::try_lock()?;
        modulators.push(name.clone());
        Ok(())
    }
    pub fn contains(name: &CName) -> Result<bool, Error> {
        let modulators = Self::try_lock()?;
        for registered in modulators.iter() {
            if registered == name {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

pub(super) trait Parameter {
    type Value;
    fn init(manager: &mut AudioManager) -> Result<(), Error>;
    fn effect() -> Result<impl EffectBuilder, Error>;
    fn update(value: Self::Value, tween: Tween) -> Result<bool, Error>;
}

/// parameter which can be updated via `AudioSystem::GlobalParameter`.
#[allow(private_bounds)]
pub trait GlobalParameter: Parameter {
    fn name() -> CName;
}
