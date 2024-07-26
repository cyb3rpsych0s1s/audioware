use audioware_bank::Banks;
use audioware_manifest::{PlayerGender, ScnDialogLineType, SpokenLocale, WrittenLocale};
use effects::{IMMEDIATELY, SMOOTHLY};
use eq::Preset;
use id::HandleId;
use kira::tween::Tween;
use manager::{Manager, StaticStorage, StreamStorage};
use red4ext_rs::{
    log,
    types::{CName, EntityId, GameInstance, Opt},
    PluginOps,
};

use scene::Scene;
use tracks::Tracks;

use crate::{
    engine::eq::EqPass,
    error::Error,
    states::State,
    types::{AsAudioSystem, AsGameInstance},
    Audioware,
};

pub mod effects;
pub mod eq;
mod id;
mod manager;
pub mod modulators;
mod scene;
mod tracks;

pub use manager::Manage;

pub struct Engine;

impl Engine {
    pub fn setup() -> Result<(), Error> {
        // SAFETY: initialization order matters
        let mut manager = Manager::try_lock()?;
        Tracks::setup(&mut manager)?;
        Scene::setup(&mut manager, &Tracks::get().v.main)?;
        Ok(())
    }
    pub fn shutdown() {
        if let Err(e) = Manager::stop(None) {
            log::error!(Audioware::env(), "could stop all sounds on manager: {e}");
        }
        if let Err(e) = Scene::clear_emitters() {
            log::error!(Audioware::env(), "could clear emitters in scene: {e}");
        }
    }
    pub fn register_emitter(entity_id: EntityId, emitter_name: Option<CName>) {
        if let Err(e) = Scene::register_emitter(entity_id, emitter_name) {
            log::error!(Audioware::env(), "couldn't register emitter to scene: {e}");
        }
    }
    pub fn unregister_emitter(entity_id: EntityId) {
        if let Err(e) = Scene::unregister_emitter(&entity_id) {
            log::error!(
                Audioware::env(),
                "couldn't unregister emitter from scene: {e}"
            );
        }
    }
    pub fn is_registered_emitter(entity_id: &EntityId) -> bool {
        Scene::is_registered_emitter(entity_id)
    }
    pub fn emitters_count() -> i32 {
        let count = Scene::emitters_count();
        if let Err(e) = count {
            log::error!(Audioware::env(), "couldn't count emitters in scene: {e}");
            return -1;
        }
        count.unwrap() as i32
    }
    pub fn clear_emitters() {
        if let Err(e) = Scene::clear_emitters() {
            log::error!(Audioware::env(), "couldn't clear emitters on scene: {e}");
        }
    }
    pub fn sync_emitters() {
        if let Err(e) = Scene::sync_emitters() {
            log::error!(Audioware::env(), "couldn't sync emitters on scene: {e}");
        }
    }
    pub fn sync_listener() {
        if let Err(e) = Scene::sync_listener() {
            log::error!(Audioware::env(), "couldn't sync listener on scene: {e}");
        }
    }
    /// play sound
    pub fn play(
        sound_name: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
        line_type: Option<ScnDialogLineType>,
        tween: Option<Tween>,
    ) {
        let mut manager = match Manager::try_lock() {
            Ok(x) => x,
            Err(e) => {
                log::error!(Audioware::env(), "Unable to get audio manager: {e}");
                return;
            }
        };
        let spoken = SpokenLocale::get();
        let written = WrittenLocale::get();
        let gender = PlayerGender::get();
        let id = match Banks::exist(&sound_name, &spoken, gender.as_ref()) {
            Ok(x) => x,
            Err(e) => {
                log::error!(Audioware::env(), "Unable to get sound ID: {e}");
                return;
            }
        };
        // TODO: output destination
        match Banks::data(id) {
            either::Either::Left(mut data) => {
                if tween.is_some() {
                    data.settings.fade_in_tween = tween;
                }
                let handle = manager.play(data).unwrap();
                match StaticStorage::try_lock() {
                    Ok(mut x) => {
                        x.insert(HandleId::new(id, entity_id), handle);
                    }
                    Err(e) => {
                        log::error!(Audioware::env(), "Unable to store static sound handle: {e}");
                    }
                }
            }
            either::Either::Right(mut data) => {
                if tween.is_some() {
                    data.settings.fade_in_tween = tween;
                }
                let handle = manager.play(data).unwrap();
                match StreamStorage::try_lock() {
                    Ok(mut x) => {
                        x.insert(HandleId::new(id, entity_id), handle);
                    }
                    Err(e) => {
                        log::error!(
                            Audioware::env(),
                            "Unable to store streaming sound handle: {e}"
                        );
                    }
                }
            }
        }
        // TODO: propagate subtitles
    }
    pub fn play_on_emitter(
        sound_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: Option<Tween>,
    ) {
        let mut manager = match Manager::try_lock() {
            Ok(x) => x,
            Err(e) => {
                log::error!(Audioware::env(), "Unable to get audio manager: {e}");
                return;
            }
        };
        let spoken = SpokenLocale::get();
        let written = WrittenLocale::get();
        let gender = PlayerGender::get();
        let id = match Banks::exist(&sound_name, &spoken, gender.as_ref()) {
            Ok(x) => x,
            Err(e) => {
                log::error!(Audioware::env(), "Unable to get sound ID: {e}");
                return;
            }
        };
        let destination = match Scene::output_destination(&entity_id) {
            Some(x) => x,
            None => {
                log::error!(
                    Audioware::env(),
                    "Entity is not registered as emitter: {entity_id:?}"
                );
                return;
            }
        };
        match Banks::data(id) {
            either::Either::Left(data) => {
                let handle = manager.play(data.output_destination(destination)).unwrap();
                match StaticStorage::try_lock() {
                    Ok(mut x) => {
                        x.insert(HandleId::new(id, Some(entity_id)), handle);
                    }
                    Err(e) => {
                        log::error!(Audioware::env(), "Unable to store static sound handle: {e}");
                    }
                }
            }
            either::Either::Right(data) => {
                let handle = manager.play(data.output_destination(destination)).unwrap();
                match StreamStorage::try_lock() {
                    Ok(mut x) => {
                        x.insert(HandleId::new(id, Some(entity_id)), handle);
                    }
                    Err(e) => {
                        log::error!(
                            Audioware::env(),
                            "Unable to store streaming sound handle: {e}"
                        );
                    }
                }
            }
        }
        // TODO: propagate subtitles
    }
    pub fn stop_by_cname(event_name: &CName, tween: Option<Tween>) {
        if let Err(e) = Manager::stop_by_cname(event_name, tween) {
            log::error!(Audioware::env(), "{e}");
        }
    }
    pub fn stop_by_cname_for_entity(
        event_name: &CName,
        entity_id: &EntityId,
        tween: Option<Tween>,
    ) {
        if let Err(e) = Manager::stop_by_cname_for_entity(event_name, entity_id, tween) {
            log::error!(Audioware::env(), "{e}");
        }
    }
    pub fn switch(
        switch_name: CName,
        switch_value: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        switch_name_tween: Option<Tween>,
        switch_value_tween: Option<Tween>,
    ) {
        let prev = Banks::exists(&switch_name);
        let next = Banks::exists(&switch_value);
        let system = GameInstance::get_audio_system();

        if prev {
            match entity_id.into_option() {
                Some(x) => Engine::stop_by_cname_for_entity(
                    &switch_name,
                    &x,
                    Some(switch_name_tween.unwrap_or(SMOOTHLY)),
                ),
                None => {
                    Engine::stop_by_cname(&switch_name, Some(switch_name_tween.unwrap_or(SMOOTHLY)))
                }
            };
        } else {
            system.stop(switch_name, entity_id, emitter_name);
        }

        if next {
            Engine::play(
                switch_value,
                entity_id.into_option(),
                emitter_name.into_option(),
                None,
                Some(switch_value_tween.unwrap_or(SMOOTHLY)),
            );
        } else {
            system.play(switch_value, entity_id, emitter_name);
        }
    }
    pub fn set_player_reverb(value: f32) {
        if !(0. ..=1.).contains(&value) {
            log::error!(
                Audioware::env(),
                "reverb must be between 0. and 1. (inclusive)"
            );
            return;
        }
        let tracks = Tracks::get();
        match tracks.reverb.try_lock() {
            Ok(ref mut x) => x.set_volume(kira::Volume::Amplitude(value as f64), IMMEDIATELY),
            Err(e) => log::error!(Audioware::env(), "Unable to set reverb volume: {e}"),
        }
    }
    pub fn set_player_preset(value: Preset) {
        let tracks = Tracks::get();
        match tracks.v.eq.try_lock() {
            Ok(ref mut x) => x.set_preset(value),
            Err(e) => log::error!(Audioware::env(), "Unable to set EQ preset: {e}"),
        }
    }
}
