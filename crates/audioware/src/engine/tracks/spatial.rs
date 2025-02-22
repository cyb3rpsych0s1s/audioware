use audioware_core::{amplitude, Amplitude, SpatialTrackSettings};
use kira::{
    backend::Backend,
    listener::ListenerId,
    track::{SpatialTrackBuilder, SpatialTrackHandle},
    AudioManager, Value,
};

use crate::{
    engine::modulators::{Modulators, Parameter},
    error::Error,
};

use super::ambience::Ambience;

pub struct Spatial(SpatialTrackHandle);

impl Spatial {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        listener: impl Into<ListenerId>,
        position: impl Into<Value<mint::Vector3<f32>>>,
        settings: SpatialTrackSettings,
        modulators: &Modulators,
        ambience: &Ambience,
    ) -> Result<Self, Error> {
        let SpatialTrackSettings {
            distances,
            persist_until_sounds_finish,
            attenuation_function,
            spatialization_strength,
        } = settings;
        let track = manager.add_spatial_sub_track(
            listener,
            position,
            SpatialTrackBuilder::new()
                .distances(distances)
                .spatialization_strength(spatialization_strength)
                .persist_until_sounds_finish(persist_until_sounds_finish)
                .attenuation_function(attenuation_function)
                // sum used to have to be 1.0 otherwise sounds crackled, what now?
                .with_send(ambience.environmental(), amplitude!(0.5).as_decibels())
                .with_send(ambience.reverb(), amplitude!(0.15).as_decibels())
                .with_effect(modulators.sfx_volume.try_effect()?),
        )?;
        Ok(Self(track))
    }
}

impl std::ops::Deref for Spatial {
    type Target = SpatialTrackHandle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Spatial {
    fn deref_mut(&mut self) -> &mut SpatialTrackHandle {
        &mut self.0
    }
}
