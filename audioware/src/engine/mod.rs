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
    crate::engine::state::update(state);
}
