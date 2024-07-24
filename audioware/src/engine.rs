use audioware_bank::Banks;
use audioware_manifest::{PlayerGender, ScnDialogLineType, SpokenLocale, WrittenLocale};
use id::HandleId;
use manager::{Manager, StaticStorage, StreamStorage};
use red4ext_rs::{
    log,
    types::{CName, EntityId},
    PluginOps,
};

use scene::Scene;
use tracks::Tracks;

use crate::{
    error::Error,
    states::State,
    types::{Quaternion, Vector4},
    Audioware,
};

mod effects;
mod eq;
mod id;
mod manager;
pub mod modulators;
mod scene;
mod tracks;

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
        if let Err(e) = Manager::stop_all() {
            log::error!(Audioware::env(), "could stop all sounds on manager: {e}");
        }
        if let Err(e) = Scene::clear_emitters() {
            log::error!(Audioware::env(), "could clear emitters in scene: {e}");
        }
    }
    pub fn register_listener(entity_id: EntityId) {
        if let Err(e) = Scene::register_listener(entity_id) {
            log::error!(Audioware::env(), "couldn't register listener to scene: {e}");
        }
    }
    pub fn update_listener(position: Vector4, orientation: Quaternion) {
        if let Err(e) = Scene::update_listener(position, orientation) {
            log::error!(Audioware::env(), "couldn't update listener in scene: {e}");
        }
    }
    pub fn unregister_listener(entity_id: EntityId) {
        if let Err(e) = Scene::unregister_listener(entity_id) {
            log::error!(
                Audioware::env(),
                "couldn't unregister listener from scene: {e}"
            );
        }
    }
    pub fn register_emitter(entity_id: EntityId, emitter_name: Option<CName>) {
        if let Err(e) = Scene::register_emitter(entity_id, emitter_name) {
            log::error!(Audioware::env(), "couldn't register emitter to scene: {e}");
        }
    }
    pub fn update_emitter(entity_id: &EntityId, position: Vector4) {
        if let Err(e) = Scene::update_emitter(entity_id, position) {
            log::error!(Audioware::env(), "couldn't update emitter in scene: {e}");
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
            either::Either::Left(data) => {
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
            either::Either::Right(data) => {
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
    pub fn play_on_emitter(sound_name: CName, entity_id: EntityId, emitter_name: CName) {
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
}
