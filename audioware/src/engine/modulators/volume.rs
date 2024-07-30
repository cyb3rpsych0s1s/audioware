mod master;
pub use master::*;
mod car_radio;
mod dialogue;
mod music;
mod radioport;
mod sfx;

macro_rules! impl_volume {
    ($struct:ident, $name:literal) => {
        const MODULATOR_NAME: &str = $name;

        static MODULATOR: std::sync::OnceLock<
            std::sync::Mutex<kira::modulator::tweener::TweenerHandle>,
        > = std::sync::OnceLock::new();

        pub struct $struct;

        impl $struct {
            fn set_once(handle: kira::modulator::tweener::TweenerHandle) {
                MODULATOR
                    .set(std::sync::Mutex::new(handle))
                    .expect("store tweener handle once")
            }
            pub fn try_lock<'a>() -> Result<
                std::sync::MutexGuard<'a, kira::modulator::tweener::TweenerHandle>,
                $crate::error::InternalError,
            > {
                MODULATOR.get().unwrap().try_lock().map_err(|_| {
                    $crate::error::InternalError::Contention {
                        origin: MODULATOR_NAME,
                    }
                })
            }
        }

        impl $crate::engine::modulators::Parameter for $struct {
            type Value = kira::Volume;

            fn setup(
                manager: &mut kira::manager::AudioManager,
            ) -> Result<(), $crate::error::Error> {
                let handle = manager.add_modulator(kira::modulator::tweener::TweenerBuilder {
                    initial_value: 100., // TODO: retrieve from game audio settings
                })?;
                Self::set_once(handle);
                Ok(())
            }

            fn effect() -> Result<impl kira::effect::EffectBuilder, $crate::error::Error> {
                use std::ops::Deref;
                Ok(kira::effect::volume_control::VolumeControlBuilder::new(
                    kira::tween::Value::<kira::Volume>::from_modulator(
                        $struct::try_lock()?.deref(),
                        kira::tween::ModulatorMapping {
                            input_range: (0.0, 100.0),
                            output_range: (
                                kira::Volume::Amplitude(0.0),
                                kira::Volume::Amplitude(100.0),
                            ),
                            clamp_bottom: true,
                            clamp_top: true,
                        },
                    ),
                ))
            }

            fn update(
                value: Self::Value,
                tween: kira::tween::Tween,
            ) -> Result<bool, $crate::error::Error> {
                Self::try_lock()?.set(value.as_amplitude(), tween);
                Ok(true)
            }
        }
    };
}
pub(super) use impl_volume;
