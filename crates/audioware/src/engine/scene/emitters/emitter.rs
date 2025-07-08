use kira::track::SpatialTrackDistances;
use red4ext_rs::types::{EntityId, GameInstance};

use crate::{
    AIActionHelper, AsEntity, AsGameInstance, AsTimeDilatable, GameObject, TimeDilatable, Vector4,
    error::{Error, SceneError},
};

use super::super::AsEntityExt;

pub struct Emitter;

impl Emitter {
    pub fn infos(entity_id: EntityId) -> Result<(Vector4, bool), Error> {
        let game = GameInstance::new();
        let entity = GameInstance::find_entity_by_id(game, entity_id);
        if entity.is_null() {
            return Err(Error::Scene {
                source: SceneError::MissingEmitter { entity_id },
            });
        }
        let busy = entity
            .clone()
            .cast::<GameObject>()
            .map(AIActionHelper::is_in_workspot)
            .unwrap_or(false);
        let position = entity.get_world_position();
        Ok((position, busy))
    }

    #[allow(clippy::type_complexity)]
    pub fn full_infos(
        entity_id: EntityId,
    ) -> Result<(Vector4, bool, Option<f32>, Option<SpatialTrackDistances>), Error> {
        let (position, busy) = Self::infos(entity_id)?;
        let game = GameInstance::new();
        let entity = GameInstance::find_entity_by_id(game, entity_id);
        if entity.is_null() {
            return Err(Error::Scene {
                source: SceneError::MissingEmitter { entity_id },
            });
        }
        let distances = entity.get_emitter_distances();
        if !entity.is_a::<TimeDilatable>() {
            return Ok((position, busy, None, distances));
        }
        let dilation = entity
            .clone()
            .cast::<TimeDilatable>()
            .as_ref()
            .map(AsTimeDilatable::get_time_dilation_value);
        Ok((position, busy, dilation, distances))
    }
}
