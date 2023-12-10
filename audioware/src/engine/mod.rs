pub use self::state::State;

mod collector;
mod id;
mod sounds;
mod state;

pub(super) fn setup() {
    collector::setup();
    sounds::setup();
}

#[inline]
pub(super) fn update_state(state: State) {
    if state == State::End || state == State::InGame {
        crate::engine::collector::unpark();
    }
    let previous = crate::engine::state::update(state);
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
