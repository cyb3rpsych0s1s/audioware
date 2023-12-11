pub use self::state::State;

mod collector;
mod manager;
mod sounds;
mod state;
mod tracks;
pub mod id;

pub(super) fn setup() {
    collector::setup();
    sounds::setup();
    manager::setup();
}

#[inline]
pub(super) fn update_state(state: State) {
    if state == State::End || state == State::InGame {
        collector::unpark();
    }
    let previous = state::update(state);
    match (previous, state) {
        (State::InGame, State::InMenu) | (State::InGame, State::InPause) => {
            sounds::pause();
        }
        (State::InMenu, State::InGame) | (State::InPause, State::InGame) => {
            sounds::resume();
        }
        _ => {}
    };
}
