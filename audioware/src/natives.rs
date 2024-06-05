use std::time::Duration;

use audioware_sys::interop::gender::PlayerGender;
use kira::tween::Tween;
use red4ext_rs::types::{CName, EntityId};

use crate::{
    engine::Manage,
    manifest::types::{ElasticTween, LinearTween},
    state::game,
    Maybe,
};

pub fn update_game_state(state: game::State) {
    crate::state::game::State::set(state);
}

pub fn update_player_gender(gender: PlayerGender) {
    if let Err(e) = crate::state::player::update_gender(gender) {
        red4ext_rs::error!("{e}");
    }
}

pub fn update_player_locales(spoken: CName, written: CName) {
    if let Err(e) = crate::state::player::update_locales(spoken, written) {
        red4ext_rs::error!("{e}");
    }
}

pub fn audioware_stop_engine() {
    let immediately = Tween {
        start_time: kira::StartTime::Immediate,
        duration: Duration::from_millis(1),
        easing: kira::tween::Easing::Linear,
    };
    crate::engine::Engine.stop(Some(immediately));
}

pub fn stop_linear(
    sound_name: CName,
    entity_id: EntityId,
    _emitter_name: CName,
    tween: LinearTween,
) {
    let tween: kira::tween::Tween = tween.into();
    match (&sound_name, entity_id.maybe()) {
        (n, None) => crate::engine::Engine.stop_by_cname(n, Some(tween)),
        (n, Some(e)) => crate::engine::Engine.stop_by_cname_for_entity(n, e, Some(tween)),
    }
}

pub fn stop_elastic(
    sound_name: CName,
    entity_id: EntityId,
    _emitter_name: CName,
    tween: ElasticTween,
) {
    let tween: kira::tween::Tween = tween.into();
    match (&sound_name, entity_id.maybe()) {
        (n, None) => crate::engine::Engine.stop_by_cname(n, Some(tween)),
        (n, Some(e)) => crate::engine::Engine.stop_by_cname_for_entity(n, e, Some(tween)),
    }
}
