use crate::natives::propagate_subtitle;

use self::sounds::SoundInfos;
pub use self::state::State;

pub mod banks;
mod id;
pub mod localization;
mod manager;
mod sounds;
mod state;
mod tracks;

pub use id::SoundId;
use kira::tween::Tween;
use red4ext_rs::types::{CName, EntityId};

pub fn setup() -> anyhow::Result<()> {
    banks::setup()?;
    sounds::setup();
    manager::setup();
    Ok(())
}

#[inline]
pub fn update_state(state: State) {
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

pub fn play(sound_name: CName, entity_id: Option<EntityId>, emitter_name: Option<CName>) {
    // let sound: &SoundId = sound_name.as_ref();
    if let Some(mut manager) = manager::try_get_mut() {
        if let Ok(data) = banks::data(&sound_name) {
            if let Some(vocal) = tracks::vocal() {
                data.settings.output_destination(vocal);
                if let Ok(handle) = manager.play(data) {
                    sounds::store(
                        handle,
                        sound_name.clone(),
                        entity_id.clone(),
                        emitter_name.clone(),
                    );
                    if let (Some(entity_id), Some(emitter_name)) = (entity_id, emitter_name) {
                        propagate_subtitle(sound_name, entity_id, emitter_name);
                    }
                } else {
                    red4ext_rs::error!("error playing sound {sound_name}");
                }
            } else {
                red4ext_rs::error!("unable to get track handle (vocal)");
            }
        } else {
            red4ext_rs::warn!("unknown sound ({sound_name})");
        }
    } else {
        red4ext_rs::error!("unable to reach audio manager");
    }
}

pub fn stop(sound_name: CName, entity_id: Option<EntityId>, emitter_name: Option<CName>) {
    if let Some(mut map) = sounds::try_get_mut() {
        for SoundInfos { handle, .. } in map.values_mut().filter(|x| {
            x.sound_name == sound_name && x.entity_id == entity_id && x.emitter_name == emitter_name
        }) {
            if handle.stop(Tween::default()).is_err() {
                red4ext_rs::warn!("unable to stop sound handle ({sound_name})");
            }
        }
    } else {
        red4ext_rs::error!("unable to reach sound handle");
    }
}

pub fn pause() -> anyhow::Result<()> {
    if let Some(mut map) = sounds::try_get_mut() {
        for SoundInfos {
            handle, sound_name, ..
        } in map.values_mut()
        {
            if handle.pause(Tween::default()).is_err() {
                red4ext_rs::warn!("unable to pause sound handle ({})", sound_name);
            }
        }
    } else {
        red4ext_rs::error!("unable to reach sound handle");
    }
    Ok(())
}

pub fn resume() -> anyhow::Result<()> {
    if let Some(mut map) = sounds::try_get_mut() {
        for SoundInfos {
            handle, sound_name, ..
        } in map.values_mut()
        {
            if handle.resume(Tween::default()).is_err() {
                red4ext_rs::warn!("unable to pause sound handle ({sound_name})");
            }
        }
    } else {
        red4ext_rs::error!("unable to reach sound handle");
    }
    Ok(())
}
