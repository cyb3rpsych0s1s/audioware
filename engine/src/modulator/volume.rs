use std::sync::{Mutex, MutexGuard};

use audioware_core::{Error, UninitializedSnafu};
use kira::{
    effect::{volume_control::VolumeControlBuilder, EffectBuilder},
    modulator::tweener::{TweenerBuilder, TweenerHandle},
    tween::{ModulatorMapping, Tween, Value},
    Volume,
};
use once_cell::sync::OnceCell;
use red4ext_rs::types::CName;
use snafu::OptionExt;

use super::{GlobalParameter, GlobalParameters, Parameter};

const MODULATOR_NAME: &str = "audioware_volume";

static MODULATOR: OnceCell<Mutex<TweenerHandle>> = OnceCell::new();

pub struct VolumeModulator;
impl VolumeModulator {
    fn set(handle: TweenerHandle) {
        let cname = CName::new_pooled(MODULATOR_NAME);
        GlobalParameters::register(&cname).expect("register global parameter name");
        MODULATOR
            .set(Mutex::new(handle))
            .expect("store tweener handle once")
    }
    fn try_lock<'a>() -> Result<MutexGuard<'a, TweenerHandle>, Error> {
        MODULATOR
            .get()
            .context(UninitializedSnafu {
                which: MODULATOR_NAME,
            })?
            .try_lock()
            .map_err(Error::from)
    }
}

impl GlobalParameter for VolumeModulator {
    fn name() -> CName {
        CName::new(MODULATOR_NAME)
    }
}
impl Parameter for VolumeModulator {
    type Value = Volume;
    fn init(manager: &mut kira::manager::AudioManager) {
        let handle = manager
            .add_modulator(TweenerBuilder { initial_value: 50. })
            .expect("instantiate volume");
        Self::set(handle);
    }

    fn update(value: Volume, tween: Tween) -> Result<bool, Error> {
        Self::try_lock()?.set(value.as_decibels(), tween);
        Ok(true)
    }

    fn effect() -> Result<impl EffectBuilder, Error> {
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
