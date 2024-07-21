use std::{
    ffi,
    sync::{Mutex, MutexGuard, OnceLock},
};

use kira::{
    effect::{volume_control::VolumeControlBuilder, EffectBuilder},
    manager::AudioManager,
    modulator::tweener::{TweenerBuilder, TweenerHandle},
    tween::{ModulatorMapping, Tween, Value},
    Volume,
};
use red4ext_rs::types::CNamePool;

use crate::error::InternalError;

use super::Parameter;

const MODULATOR_NAME: &str = "Audioware:Volume";

static MODULATOR: OnceLock<Mutex<TweenerHandle>> = OnceLock::new();

pub struct VolumeModulator;

impl VolumeModulator {
    fn set_once(handle: TweenerHandle) {
        CNamePool::add_cstr(
            ffi::CString::new(MODULATOR_NAME)
                .expect("internally defined")
                .as_c_str(),
        );
        MODULATOR
            .set(Mutex::new(handle))
            .expect("store tweener handle once")
    }
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

impl Parameter for VolumeModulator {
    type Value = Volume;
    fn setup(manager: &mut AudioManager) -> Result<(), crate::Error> {
        let handle = manager.add_modulator(TweenerBuilder { initial_value: 50. })?;
        Self::set_once(handle);
        Ok(())
    }

    fn update(value: Volume, tween: Tween) -> Result<bool, crate::Error> {
        Self::try_lock()?.set(value.as_decibels(), tween);
        Ok(true)
    }

    fn effect() -> Result<impl EffectBuilder, crate::Error> {
        Ok(VolumeControlBuilder::new(Value::from_modulator(
            &*Self::try_lock()?,
            ModulatorMapping {
                input_range: (0.0, 100.0),
                output_range: (
                    Volume::Decibels(Volume::MIN_DECIBELS),
                    Volume::Decibels(70.),
                ),
                clamp_bottom: true,
                clamp_top: true,
            },
        )))
    }
}
