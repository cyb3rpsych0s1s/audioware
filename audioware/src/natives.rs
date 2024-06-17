//! natives used across Redscript ↔️ Rust bridge

use std::time::Duration;

use audioware_core::{audioware_error, ok_or_return};
use audioware_engine::{track::TrackName, update_gender, update_locales, Engine, State};
use audioware_manifest::{AsChildTween, AudiowareTween, IntoTween};
use audioware_sys::interop::gender::PlayerGender;
use kira::tween::Tween;
use red4ext_rs::types::{CName, EntityId, MaybeUninitRef};

use crate::Maybe;

pub fn update_game_state(state: State) {
    audioware_engine::Engine::update_game_state(state);
}

pub fn update_volume(value: f32) -> bool {
    ok_or_return!(Engine::update_volume(value), false)
}

pub fn update_player_gender(gender: PlayerGender) {
    if let Err(e) = update_gender(gender) {
        red4ext_rs::error!("{e}");
    }
}

pub fn update_player_locales(spoken: CName, written: CName) {
    if let Err(e) = update_locales(spoken, written) {
        red4ext_rs::error!("{e}");
    }
}

pub fn audioware_stop_engine() {
    let immediately = Tween {
        start_time: kira::StartTime::Immediate,
        duration: Duration::from_millis(1),
        easing: kira::tween::Easing::Linear,
    };
    Engine::stop(Some(immediately));
}

/// stop sound playing on track
///
/// used in conjunction with [AudioSystem::Play](https://nativedb.red4ext.com/gameGameAudioSystem#Play).
///
/// SAFETY:
/// * `entity_id`    - `opt`ional in Redscript.
/// * `emitter_name` - `opt`ional in Redscript.
/// * `tween`        - `ref<T>` can always be `null` in Redscript.
pub fn audioware_track_stop(
    sound_name: CName,
    entity_id: EntityId,
    _emitter_name: CName,
    tween: MaybeUninitRef<AudiowareTween>,
) {
    match (&sound_name, entity_id.maybe()) {
        (n, None) => Engine::stop_by_cname(n, convert_tween(tween)),
        (n, Some(e)) => Engine::stop_by_cname_for_entity(n, e, convert_tween(tween)),
    }
}

#[red4ext_rs::macros::redscript_global(name = "Audioware.DelegatePlay")]
pub fn delegate_play(sound_name: CName, entity_id: EntityId, emitter_name: CName) -> ();

#[red4ext_rs::macros::redscript_global(name = "Audioware.DelegateStop")]
pub fn delegate_stop(sound_name: CName, entity_id: EntityId, emitter_name: CName) -> ();

pub fn audioware_add_track(name: TrackName) {
    if let Err(e) = Engine::add_sub_track(&name) {
        audioware_error!("{e}");
    }
}

pub fn audioware_remove_track(name: TrackName) {
    if let Err(e) = Engine::remove_sub_track(&name) {
        audioware_error!("{e}");
    }
}
pub fn audioware_play_on_track(
    sound_name: CName,
    track_name: TrackName,
    entity_id: EntityId,
    emitter_name: CName,
    tween: MaybeUninitRef<AudiowareTween>,
) {
    if let Err(e) = Engine::play_on_track(
        &sound_name,
        &track_name,
        entity_id.maybe(),
        emitter_name.maybe(),
        convert_tween(tween),
    ) {
        audioware_error!("{e}");
    }
}

fn convert_tween(tween: MaybeUninitRef<AudiowareTween>) -> Option<Tween> {
    if let Some(tween) = tween.into_ref() {
        match (tween.linear(), tween.elastic()) {
            (None, None) => {
                red4ext_rs::error!("unknown tween");
                return None;
            }
            (None, Some(x)) => return Some(x.into_tween()),
            (Some(x), None) => return Some(x.into_tween()),
            (Some(_), Some(_)) => {
                red4ext_rs::error!("ambiguous tween");
                return None;
            }
        }
    }
    None
}
