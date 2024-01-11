use crate::natives::propagate_subtitle;

use self::sounds::SoundInfos;
pub use self::state::State;

pub mod banks;
pub mod id;
pub mod localization;
pub mod manager;
pub mod sounds;
pub mod state;
pub mod tracks;

use audioware_sys::interop::{quaternion::Quaternion, vector4::Vector4};
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

/// play sound
pub fn play(sound_name: CName, entity_id: Option<EntityId>, emitter_name: Option<CName>) {
    if let Some(mut manager) = manager::try_get_mut() {
        if let Ok(mut data) = banks::data(&sound_name) {
            if let Some(destination) =
                tracks::output_destination(entity_id.clone(), emitter_name.clone())
            {
                data.settings.output_destination = destination;
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
                red4ext_rs::error!("unable to get sound output destination");
            }
        } else {
            red4ext_rs::warn!("unknown sound ({sound_name})");
        }
    } else {
        red4ext_rs::error!("unable to reach audio manager");
    }
}

/// stop sound(s) matching provided parameters
///
/// iterate through all the values of the sounds pool,
/// matching on `sound_name`, `entity_id` and `emitter_name`
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

pub fn update_actor_location(id: EntityId, position: Vector4, orientation: Quaternion) {
    let gi = audioware_sys::interop::game::get_game_instance();
    let entity = audioware_sys::interop::game::find_entity_by_id(gi, id.clone());
    let is = entity
        .into_ref()
        .map(|entity| entity.is_player())
        .unwrap_or(false);
    if is {
        tracks::update_listener(position, orientation);
    } else {
        tracks::update_emitter(id, position);
    }
}
