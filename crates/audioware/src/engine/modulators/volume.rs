mod car_radio;

mod dialogue;
mod music;
mod radioport;
mod sfx;

pub const VOLUME_MAPPING: kira::Mapping<kira::Decibels> = kira::Mapping {
    input_range: (0.0, 1.0),
    output_range: (kira::Decibels::SILENCE, kira::Decibels::IDENTITY),
    easing: kira::Easing::OutPowf(3.0), // more realistic volume scaling
};

macro_rules! impl_volume {
    ($struct:ident) => {
        pub struct $struct(::kira::modulator::tweener::TweenerHandle);
        impl $crate::engine::modulators::Parameter for $struct {
            type Value = f64;
            fn try_new<B: ::kira::backend::Backend>(
                manager: &mut ::kira::AudioManager<B>,
            ) -> Result<Self, $crate::error::Error> {
                let handle = manager.add_modulator(kira::modulator::tweener::TweenerBuilder {
                    initial_value: 1., // here, RTTI hasn't loaded yet
                })?;
                Ok(Self(handle))
            }
            fn try_effect(
                &self,
            ) -> Result<impl ::kira::effect::EffectBuilder, $crate::error::Error> {
                Ok(kira::effect::volume_control::VolumeControlBuilder::new(
                    kira::Value::<kira::Decibels>::from_modulator(
                        &self.0,
                        crate::engine::modulators::VOLUME_MAPPING,
                    ),
                ))
            }
            fn update(&mut self, value: Self::Value, tween: ::kira::Tween) {
                self.0.set(value, tween);
            }
        }
    };
}
pub(super) use impl_volume;
