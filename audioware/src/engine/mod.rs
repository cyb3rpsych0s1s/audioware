pub use self::state::State;

mod banks;
mod collector;
mod id;
mod manager;
mod sounds;
mod state;
mod tracks;

pub use id::SoundId;

pub(super) fn setup() -> anyhow::Result<()> {
    banks::setup()?;
    collector::setup();
    sounds::setup();
    manager::setup();
    Ok(())
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
