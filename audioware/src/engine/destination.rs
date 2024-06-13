use audioware_sys::interop::entity::{find_entity_by_id, Entity};
use audioware_sys::interop::game::get_game_instance;
use kira::OutputDestination;
use red4ext_rs::types::{CName, EntityId};

use super::{error::Error, track::maybe_tracks};

pub fn output_destination(
    entity_id: Option<&EntityId>,
    emitter_name: Option<&CName>,
    over_the_phone: bool,
) -> Result<OutputDestination, Error> {
    let is_player = entity_id
        .and_then(|x| {
            let gi = get_game_instance();
            let entity = find_entity_by_id(gi, x.clone());
            entity.into_ref().as_ref().map(Entity::is_player)
        })
        .unwrap_or(false);
    match (entity_id, emitter_name, is_player, over_the_phone) {
        (Some(_), _, true, _) => Ok(OutputDestination::from(&maybe_tracks()?.v.vocal)),
        (Some(_), _, false, false) => Ok(OutputDestination::from(&maybe_tracks()?.v.environmental)),
        (None, _, false, true) => Ok(OutputDestination::from(&maybe_tracks()?.holocall.main)),
        _ => Ok(OutputDestination::from(&maybe_tracks()?.v.main)),
    }
}
