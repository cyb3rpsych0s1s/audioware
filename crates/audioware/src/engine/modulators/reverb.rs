use kira::{
    backend::Backend,
    effect::{reverb::ReverbBuilder, EffectBuilder},
    modulator::tweener::{TweenerBuilder, TweenerHandle},
    Mapping, Mix, Tween, Value,
};

use super::Parameter;

/// Reverb mix parameter.
pub struct ReverbMix(TweenerHandle);
impl Parameter for ReverbMix {
    type Value = f32;

    fn try_new<B: Backend>(
        manager: &mut kira::AudioManager<B>,
    ) -> Result<Self, crate::error::Error> {
        let handle = manager.add_modulator(TweenerBuilder { initial_value: 0.0 })?;
        Ok(Self(handle))
    }

    fn try_effect(&self) -> Result<impl EffectBuilder, crate::error::Error> {
        Ok(ReverbBuilder::new()
            .stereo_width(1.0)
            .mix(Value::from_modulator(
                &self.0,
                Mapping {
                    input_range: (0.0, 1.0),
                    output_range: (Mix::DRY, Mix::WET),
                    easing: kira::Easing::Linear,
                },
            )))
    }

    fn update(&mut self, value: Self::Value, tween: Tween) {
        self.0.set(value as f64, tween);
    }
}
