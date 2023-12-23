use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard, OnceLock},
};

use kira::{sound::static_sound::StaticSoundHandle, tween::Tween};
use lazy_static::lazy_static;
use red4ext_rs::types::{CName, EntityId};
use ulid::Ulid;

use super::id::SoundId;

pub struct SoundInfos {
    pub sound_name: CName,
    pub entity_id: Option<EntityId>,
    pub emitter_name: Option<CName>,
    pub handle: StaticSoundHandle,
}

lazy_static! {
    static ref SOUNDS_POOL: OnceLock<Mutex<HashMap<Ulid, SoundInfos>>> = OnceLock::default();
}

pub fn setup() {
    if SOUNDS_POOL.set(Mutex::new(HashMap::default())).is_err() {
        red4ext_rs::error!("error initializing sounds pool");
    }
}

pub fn store(
    _id: SoundId,
    handle: StaticSoundHandle,
    sound_name: CName,
    entity_id: Option<EntityId>,
    emitter_name: Option<CName>,
) {
    if let Some(mut pool) = SOUNDS_POOL.get().and_then(|x| x.try_lock().ok()) {
        pool.insert(
            Ulid::new(),
            SoundInfos {
                handle,
                sound_name,
                entity_id,
                emitter_name,
            },
        );
    }
}

pub fn try_get_mut<'a>() -> Option<MutexGuard<'a, HashMap<Ulid, SoundInfos>>> {
    SOUNDS_POOL.get().and_then(|x| x.try_lock().ok())
}

pub fn pause() {
    if let Some(mut pool) = SOUNDS_POOL.get().and_then(|x| x.try_lock().ok()) {
        pool.values_mut().for_each(
            |SoundInfos {
                 ref sound_name,
                 handle,
                 ..
             }| {
                if handle.pause(Tween::default()).is_err() {
                    red4ext_rs::warn!("unable to pause sound handle ({sound_name})");
                }
            },
        );
    }
}

pub fn resume() {
    if let Some(mut pool) = SOUNDS_POOL.get().and_then(|x| x.try_lock().ok()) {
        pool.values_mut().for_each(
            |SoundInfos {
                 ref sound_name,
                 handle,
                 ..
             }| {
                if handle.resume(Tween::default()).is_err() {
                    red4ext_rs::warn!("unable to resume sound handle ({sound_name})");
                }
            },
        );
    }
}
