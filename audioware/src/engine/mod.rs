pub use self::state::State;

mod banks;
mod collector;
mod id;
pub(crate) mod localization;
mod manager;
mod sounds;
mod state;
mod tracks;

pub use id::SoundId;
use red4ext_rs::types::CName;

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

pub(crate) fn play(sound: CName) {
    let sound: SoundId = sound.into();
    if let Some(mut manager) = manager::try_get_mut() {
        if let Ok(data) = banks::data(sound.clone()) {
            if let Some(vocal) = tracks::vocal() {
                data.settings.output_destination(vocal);
                if let Ok(handle) = manager.play(data) {
                    sounds::store(sound, handle);
                } else {
                    red4ext_rs::error!("error playing sound {sound}");
                }
            } else {
                red4ext_rs::error!("unable to get track handle (vocal)");
            }
        } else {
            red4ext_rs::warn!("unknown sound ({sound})");
        }
    } else {
        red4ext_rs::error!("unable to reach audio manager");
    }
}

pub(crate) fn stop(sound: CName) -> anyhow::Result<()> {
    let sound: SoundId = sound.into();
    if let Some(mut map) = sounds::try_get_mut() {
        if let Some(handle) = map.get_mut(&sound) {
            if handle.stop(Default::default()).is_err() {
                red4ext_rs::warn!("unable to stop sound handle ({sound})");
            }
        } else {
            red4ext_rs::warn!("unknown sound handle ({sound})");
        }
    } else {
        red4ext_rs::error!("unable to reach sound handle ({sound})");
    }
    Ok(())
}

pub(crate) fn pause(sound: CName) -> anyhow::Result<()> {
    let sound: SoundId = sound.into();
    if let Some(mut map) = sounds::try_get_mut() {
        if let Some(handle) = map.get_mut(&sound) {
            if handle.pause(Default::default()).is_err() {
                red4ext_rs::warn!("unable to pause sound handle ({sound})");
            }
        } else {
            red4ext_rs::warn!("unknown sound handle ({sound})");
        }
    } else {
        red4ext_rs::error!("unable to reach sound handle ({sound})");
    }
    Ok(())
}

pub(crate) fn resume(sound: CName) -> anyhow::Result<()> {
    let sound: SoundId = sound.into();
    if let Some(mut map) = sounds::try_get_mut() {
        if let Some(handle) = map.get_mut(&sound) {
            if handle.resume(Default::default()).is_err() {
                red4ext_rs::warn!("unable to resume sound handle ({sound})");
            }
        } else {
            red4ext_rs::warn!("unknown sound handle ({sound})");
        }
    } else {
        red4ext_rs::error!("unable to reach sound handle ({sound})");
    }
    Ok(())
}
