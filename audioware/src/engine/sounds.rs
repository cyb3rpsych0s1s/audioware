use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};

use crate::types::error::{Error, InternalError};
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

#[inline(always)]
pub(crate) fn maybe_sounds<'guard>() -> Result<MutexGuard<'guard, HashMap<Ulid, SoundInfos>>, Error>
{
    sounds_pool().try_lock().map_err(|_| {
        Error::from(InternalError::Contention {
            origin: "sounds pool",
        })
    })
}

pub fn store(
    handle: StaticSoundHandle,
    sound_name: CName,
    entity_id: Option<EntityId>,
    emitter_name: Option<CName>,
) -> Result<(), Error> {
    let mut pool = maybe_sounds()?;
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
    Ok(())
}

pub fn pause() {
    if let Ok(mut pool) = maybe_sounds() {
        pool.values_mut().for_each(|SoundInfos { handle, .. }| {
            handle.pause(Tween::default());
        });
    }
}

pub fn resume() {
    if let Ok(mut pool) = maybe_sounds() {
        pool.values_mut().for_each(|SoundInfos { handle, .. }| {
            handle.resume(Tween::default());
        });
    }
}
