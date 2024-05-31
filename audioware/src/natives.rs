use std::time::Duration;

use audioware_sys::interop::gender::PlayerGender;
use kira::tween::Tween;
use red4ext_rs::types::CName;

use crate::{engine::Manage, state::game};

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
