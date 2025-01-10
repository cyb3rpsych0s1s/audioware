use audioware_core::SpatialTrackSettings;
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

pub struct Spatial(SpatialTrackHandle);

impl Spatial {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        listener: impl Into<ListenerId>,
        position: impl Into<Value<mint::Vector3<f32>>>,
        settings: SpatialTrackSettings,
        modulators: &Modulators,
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
                .spatialization_strength(spatialization_strength.0)
                .persist_until_sounds_finish(persist_until_sounds_finish)
                .attenuation_function(attenuation_function)
                .with_effect(modulators.sfx_volume.try_effect()?),
        )?;
        Ok(Self(track))
    }
}
