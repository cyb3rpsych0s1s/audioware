use audioware_sys::interop::entity::find_entity_by_id;
use audioware_sys::interop::game::get_game_instance;
use kira::OutputDestination;
use red4ext_rs::types::{CName, EntityId};
use snafu::OptionExt;

use crate::engine::{error::CannotFindEntitySnafu, id::SoundEntityId, scene::maybe_scene_entities};

use super::{error::Error, track::maybe_tracks};

pub fn output_destination(
    entity_id: Option<EntityId>,
    emitter_name: Option<CName>,
    over_the_phone: bool,
) -> Result<OutputDestination, Error> {
    let is_player = entity_id
        .clone()
        .and_then(|x| {
            let gi = get_game_instance();
            let entity = find_entity_by_id(gi, x);
            entity.into_ref().map(|entity| entity.is_player())
        })
        .unwrap_or(false);
    match (entity_id, emitter_name, is_player, over_the_phone) {
        (Some(_), Some(_), true, _) => Ok(OutputDestination::from(&maybe_tracks()?.v.vocal)),
        (Some(_), None, true, _) => Ok(OutputDestination::from(&maybe_tracks()?.v.emissive)),
        (Some(id), _, false, _) => {
            red4ext_rs::info!(
                "retrieving entity id from scene ({})",
                u64::from(id.clone())
            );
            maybe_scene_entities()?
                .get(&SoundEntityId(id.clone()))
                .map(OutputDestination::from)
                .context(CannotFindEntitySnafu {
                    entity_id: id.clone(),
                })
        }
        (None, Some(_), false, true) => Ok(OutputDestination::from(&maybe_tracks()?.holocall.main)),
        (None, _, _, _) => Ok(OutputDestination::from(&maybe_tracks()?.v.main)),
    }
}
