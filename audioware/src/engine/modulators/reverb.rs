use std::sync::{Mutex, MutexGuard, OnceLock};

use kira::{
    effect::{reverb::ReverbBuilder, EffectBuilder},
    modulator::tweener::{TweenerBuilder, TweenerHandle},
    tween::{ModulatorMapping, Tween, Value},
};

use crate::error::InternalError;

use super::Parameter;

const MODULATOR_NAME: &str = "ReverbMix";
static MODULATOR: OnceLock<Mutex<TweenerHandle>> = OnceLock::new();

pub struct ReverbMix;
impl ReverbMix {
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
impl Parameter for ReverbMix {
    type Value = f32;

    fn setup(manager: &mut kira::manager::AudioManager) -> Result<(), crate::error::Error> {
        let handle = manager.add_modulator(TweenerBuilder { initial_value: 0.0 })?;
        MODULATOR
            .set(Mutex::new(handle))
            .expect("store tweener handle once");
        Ok(())
    }

    fn effect() -> Result<impl EffectBuilder, crate::error::Error> {
        use std::ops::Deref;
        Ok(ReverbBuilder::new().mix(Value::<f64>::from_modulator(
            Self::try_lock()?.deref(),
            ModulatorMapping {
                input_range: (0.0, 1.0),
                output_range: (0.0, 1.0),
                clamp_bottom: true,
                clamp_top: true,
            },
        )))
    }

    fn update(value: Self::Value, tween: Tween) -> Result<bool, crate::error::Error> {
        Self::try_lock()?.set(value as f64, tween);
        Ok(true)
    }
}
