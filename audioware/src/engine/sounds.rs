use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard, OnceLock},
};

use kira::{
    sound::{static_sound::StaticSoundHandle, PlaybackState},
    tween::Tween,
};
use lazy_static::lazy_static;
use red4ext_rs::types::{CName, EntityId};
use ulid::Ulid;

pub struct SoundInfos {
    pub sound_name: CName,
    pub entity_id: Option<EntityId>,
    pub emitter_name: Option<CName>,
    pub handle: StaticSoundHandle,
}

impl SoundInfos {
    pub fn finished(&self) -> bool {
        self.handle.state() == PlaybackState::Stopped
    }
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
    handle: StaticSoundHandle,
    sound_name: CName,
    entity_id: Option<EntityId>,
    emitter_name: Option<CName>,
) {
    if let Some(mut pool) = SOUNDS_POOL.get().and_then(|x| x.try_lock().ok()) {
        let infos = SoundInfos {
            handle,
            sound_name,
            entity_id,
            emitter_name,
        };
        if let Some(reuse) = pool.values_mut().find(|x| x.finished()) {
            *reuse = infos;
        } else {
            pool.insert(Ulid::new(), infos);
        }
    } else {
        red4ext_rs::error!("unable to reach sounds pool");
    }
}

pub fn try_get_mut<'a>() -> Option<MutexGuard<'a, HashMap<Ulid, SoundInfos>>> {
    SOUNDS_POOL.get().and_then(|x| x.try_lock().ok())
}

pub fn pause() {
    if let Some(mut pool) = try_get_mut() {
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
    if let Some(mut pool) = try_get_mut() {
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
