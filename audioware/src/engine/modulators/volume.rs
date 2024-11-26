mod car_radio;
pub use car_radio::*;
mod dialogue;
pub use dialogue::*;
mod music;
pub use music::*;
mod radioport;
pub use radioport::*;
mod sfx;
pub use sfx::*;

macro_rules! impl_volume {
    ($struct:ident) => {
        pub struct $struct(::kira::modulator::tweener::TweenerHandle);
        impl $crate::engine::modulators::Parameter for $struct {
            type Value = f64;
            fn try_new<B: ::kira::manager::backend::Backend>(
                manager: &mut ::kira::manager::AudioManager<B>,
            ) -> Result<Self, $crate::error::Error> {
                let handle = manager.add_modulator(kira::modulator::tweener::TweenerBuilder {
                    initial_value: 100., // here, RTTI hasn't loaded yet
                })?;
                Ok(Self(handle))
            }
            fn try_effect(
                &self,
            ) -> Result<impl ::kira::effect::EffectBuilder, $crate::error::Error> {
                Ok(kira::effect::volume_control::VolumeControlBuilder::new(
                    kira::tween::Value::<kira::Volume>::from_modulator(
                        &self.0,
                        kira::tween::ModulatorMapping {
                            input_range: (0.0, 100.0),
                            output_range: (
                                kira::Volume::Amplitude(0.0),
                                kira::Volume::Amplitude(1.0),
                            ),
                            clamp_bottom: true,
                            clamp_top: true,
                        },
                    ),
                ))
            }
            fn update(&mut self, value: Self::Value, tween: ::kira::tween::Tween) {
                self.0.set(value, tween);
            }
        }
    };
}
pub(super) use impl_volume;
