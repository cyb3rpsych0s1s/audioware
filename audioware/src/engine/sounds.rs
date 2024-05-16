use std::{collections::HashMap, sync::Mutex};

use kira::{
    sound::{static_sound::StaticSoundHandle, PlaybackState},
    tween::Tween,
};
use once_cell::sync::OnceCell;
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

pub(super) fn sounds_pool() -> &'static Mutex<HashMap<Ulid, SoundInfos>> {
    static INSTANCE: OnceCell<Mutex<HashMap<Ulid, SoundInfos>>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

pub fn store(
    handle: StaticSoundHandle,
    sound_name: CName,
    entity_id: Option<EntityId>,
    emitter_name: Option<CName>,
) {
    if let Ok(mut pool) = self::sounds_pool().try_lock() {
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

pub fn pause() {
    if let Ok(mut pool) = self::sounds_pool().try_lock() {
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
    if let Ok(mut pool) = self::sounds_pool().try_lock() {
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
