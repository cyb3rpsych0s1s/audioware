use kira::{
    effect::{reverb::ReverbBuilder, EffectBuilder},
    manager::backend::Backend,
    modulator::tweener::{TweenerBuilder, TweenerHandle},
    tween::{ModulatorMapping, Tween, Value},
};

use super::Parameter;

/// Reverb mix parameter.
pub struct ReverbMix(TweenerHandle);
impl Parameter for ReverbMix {
    type Value = f32;

    fn try_new<B: Backend>(
        manager: &mut kira::manager::AudioManager<B>,
    ) -> Result<Self, crate::error::Error> {
        let handle = manager.add_modulator(TweenerBuilder { initial_value: 0.0 })?;
        Ok(Self(handle))
    }

    fn try_effect(&self) -> Result<impl EffectBuilder, crate::error::Error> {
        Ok(ReverbBuilder::new()
            .stereo_width(1.0)
            .mix(Value::<f64>::from_modulator(
                &self.0,
                ModulatorMapping {
                    input_range: (0.0, 1.0),
                    output_range: (0.0, 1.0),
                    clamp_bottom: true,
                    clamp_top: true,
                },
            )))
    }

    fn update(&mut self, value: Self::Value, tween: Tween) {
        self.0.set(value as f64, tween);
    }
}
