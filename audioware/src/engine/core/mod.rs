pub use self::state::State;

mod collector;
mod sounds;
mod state;

pub(crate) fn update_state(state: State) {
    if state == State::End || state == State::InGame {
        collector::unpark();
    }
    state::update(state);
}