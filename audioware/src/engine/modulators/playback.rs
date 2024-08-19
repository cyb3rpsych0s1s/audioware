use std::sync::{Mutex, MutexGuard, OnceLock};

use kira::{
    manager::AudioManager,
    modulator::tweener::{TweenerBuilder, TweenerHandle},
    sound::PlaybackRate,
    spatial::scene::SpatialSceneHandle,
    tween::{ModulatorMapping, Tween, Value},
};

use crate::error::InternalError;

const MODULATOR_NAME: &str = "TimeDilationPlayback";
static MODULATOR: OnceLock<Mutex<TweenerHandle>> = OnceLock::new();

pub struct TimeDilationPlayback;
impl TimeDilationPlayback {
    pub fn try_lock<'a>() -> Result<MutexGuard<'a, TweenerHandle>, InternalError> {
        MODULATOR
            .get()
            .unwrap()
            .try_lock()
            .map_err(|_| InternalError::Contention {
                origin: MODULATOR_NAME,
            })
    }
}

impl TimeDilationPlayback {
    pub(super) fn setup(manager: &mut AudioManager) -> Result<(), crate::error::Error> {
        let handle = manager.add_modulator(TweenerBuilder { initial_value: 0.0 })?;
        MODULATOR
            .set(Mutex::new(handle))
            .expect("store tweener handle once");

        Ok(())
    }
}
