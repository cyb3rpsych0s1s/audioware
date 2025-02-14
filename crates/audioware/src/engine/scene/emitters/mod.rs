use std::{collections::HashSet, sync::LazyLock};

use audioware_bank::{BankData, Banks, Id};
use audioware_core::With;
use audioware_manifest::ValidateFor;
use dashmap::{
    mapref::{multiple::RefMutMulti, one::RefMut},
    DashMap,
};
use either::Either;
use kira::{
    sound::{
        static_sound::{StaticSoundData, StaticSoundHandle},
        streaming::{StreamingSoundData, StreamingSoundHandle},
        FromFileError,
    },
    Tween,
};
use parking_lot::RwLock;
use red4ext_rs::types::{CName, EntityId};
use slot::EmitterSlot;
use slots::EmitterSlots;

use crate::{
    engine::{tracks::Spatial, tweens::IMMEDIATELY},
    error::{EngineError, Error, SceneError},
    utils::{lifecycle, warns},
    Vector4,
};

mod emitter;
mod handles;
mod slot;
mod slots;

pub use emitter::Emitter;

use super::AffectedByTimeDilation;

#[allow(clippy::type_complexity)]
pub static EMITTERS: LazyLock<RwLock<HashSet<(EntityId, CName)>>> =
    LazyLock::new(|| RwLock::new(HashSet::new()));

pub struct Emitters(DashMap<EntityId, EmitterSlots>);

impl Emitters {
    pub fn with_capacity(capacity: usize) -> Self {
        *EMITTERS.write() = HashSet::with_capacity(capacity);
        Self(Default::default())
    }
    pub fn exists_tag(&self, entity_id: &EntityId, tag_name: &CName) -> bool {
        self.0
            .get(entity_id)
            .map(|x| x.exists_tag(tag_name))
            .unwrap_or(false)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn add_emitter(
        &mut self,
        handle: Spatial,
        entity_id: EntityId,
        tag_name: CName,
        emitter_name: Option<CName>,
        dilation: Option<f32>,
        last_known_position: Vector4,
        busy: bool,
        persist_until_sounds_finish: bool,
    ) -> Result<(), Error> {
        if self.exists_tag(&entity_id, &tag_name) {
            warns!(
                "emitter {entity_id} with tag name {} was already registered",
                tag_name.as_str()
            );
            return Ok(());
        }
        let slot = EmitterSlot::new(handle, tag_name, emitter_name, persist_until_sounds_finish);
        if let Some(mut slots) = self.0.get_mut(&entity_id) {
            slots.value_mut().insert(slot);
        } else {
            self.0.insert(
                entity_id,
                EmitterSlots::new(slot, dilation, busy, last_known_position),
            );
        }
        EMITTERS.write().insert((entity_id, tag_name));
        lifecycle!(
            "added emitter {entity_id} with tag name {}",
            tag_name.as_str()
        );
        Ok(())
    }
    pub fn sync_emitters(&mut self) -> Result<(), Error> {
        if self.0.is_empty() {
            return Ok(());
        }
        self.0.retain(|k, v| {
            if v.marked_for_death && !v.any_playing_handle() {
                EMITTERS.write().retain(|(id, _)| id != k);
                return false;
            }
            let Ok((position, busy)) = Emitter::infos(*k) else {
                EMITTERS.write().retain(|(id, _)| id != k);
                return false;
            };
            v.busy = busy;
            v.last_known_position = position;
            // weirdly enough if emitter is not updated, sound(s) won't update as expected.
            // e.g. when listener moves but emitter stands still.
            v.set_emitter_position(position);
            true
        });
        Ok(())
    }
    pub fn unregister_emitter(&mut self, entity_id: &EntityId, tag_name: &CName) -> bool {
        let mut last = false;
        let mut removed = false;
        if let Some(mut slots) = self.0.get_mut(entity_id) {
            slots.value_mut().unregister_emitter(tag_name);
            last = slots.is_empty();
            removed = true;
        }
        if removed && last {
            self.0.remove(entity_id);
        }
        if removed {
            EMITTERS
                .write()
                .retain(|(id, name)| id != entity_id || name != tag_name);
        }
        removed
    }
    pub fn on_emitter_dies(&mut self, entity_id: &EntityId) {
        self.0.retain(|k, v| {
            if k == entity_id {
                let mut retain = false;
                v.slots.retain_mut(|x| {
                    if x.persist_until_sounds_finish {
                        retain = true;
                        true
                    } else {
                        x.stop(IMMEDIATELY);
                        false
                    }
                });
                if retain {
                    v.marked_for_death = true;
                }
                retain
            } else {
                true
            }
        });
        EMITTERS.write().retain(|(id, _)| id != entity_id);
    }
    pub fn on_emitter_incapacitated(&mut self, entity_id: EntityId, tween: Tween) {
        if let Some(mut slots) = self.0.get_mut(&entity_id) {
            for ref mut slot in slots.slots.iter_mut() {
                if !slot.persist_until_sounds_finish {
                    slot.stop(tween);
                }
            }
        }
    }
    pub fn stop_on_emitter(
        &mut self,
        event_name: CName,
        entity_id: EntityId,
        tag_name: CName,
        tween: Tween,
    ) {
        if let Some(mut slots) = self.0.get_mut(&entity_id) {
            slots
                .value_mut()
                .stop_on_emitter(event_name, tag_name, tween);
        }
    }
    pub fn play_on_emitter<T>(
        &mut self,
        key: &Id,
        banks: &Banks,
        event_name: CName,
        entity_id: EntityId,
        tag_name: CName,
        ext: Option<T>,
    ) -> Result<(f32, Option<CName>), Error>
    where
        StaticSoundData: With<Option<T>>,
        StreamingSoundData<FromFileError>: With<Option<T>>,
        T: AffectedByTimeDilation
            + ValidateFor<Either<StaticSoundData, StreamingSoundData<FromFileError>>>,
    {
        let Some(mut slots) = self.get_mut(&entity_id) else {
            return Err(SceneError::MissingEmitter { entity_id }.into());
        };
        let Some(slot) = slots.get_mut(&tag_name) else {
            return Err(SceneError::MissingEmitter { entity_id }.into());
        };
        let data = banks.data(key);
        if let Some(Err(e)) = ext.as_ref().map(|x| x.validate_for(&data)) {
            return Err(Error::Validation { errors: e });
        }
        let dilatable = ext
            .as_ref()
            .map(AffectedByTimeDilation::affected_by_time_dilation)
            .unwrap_or(true);
        slot.play_and_store(event_name, dilatable, data)
            .map_err(|e| match e {
                Either::Left(e) => Error::Engine {
                    source: EngineError::Sound { source: e },
                },
                Either::Right(e) => Error::Engine {
                    source: EngineError::FromFile { source: e },
                },
            })
    }
    pub fn stop_emitters(&mut self, tween: Tween) {
        self.0.iter_mut().for_each(|mut x| {
            x.value_mut().stop(tween);
        });
    }
    pub fn pause(&mut self, tween: Tween) {
        self.0.iter_mut().for_each(|mut x| {
            x.value_mut().pause(tween);
        });
    }
    pub fn resume(&mut self, tween: Tween) {
        self.0.iter_mut().for_each(|mut x| {
            x.value_mut().resume(tween);
        });
    }
    pub fn get_mut(&mut self, entity_id: &EntityId) -> Option<RefMut<'_, EntityId, EmitterSlots>> {
        self.0.get_mut(entity_id)
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = RefMutMulti<'_, EntityId, EmitterSlots>> {
        self.0.iter_mut()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn clear(&mut self) {
        EMITTERS.write().clear();
        self.0.clear();
    }
    pub fn reclaim(&mut self) {
        self.0.iter_mut().for_each(|mut x| x.reclaim());
    }
}

impl Drop for Emitters {
    fn drop(&mut self) {
        EMITTERS.write().clear();
    }
}

pub trait Store<T> {
    fn store(
        &mut self,
        tag_name: CName,
        event_name: CName,
        handle: T,
        affected_by_time_dilation: bool,
    );
}

impl Store<Either<StaticSoundHandle, StreamingSoundHandle<FromFileError>>> for Emitters {
    fn store(
        &mut self,
        tag_name: CName,
        event_name: CName,
        handle: Either<StaticSoundHandle, StreamingSoundHandle<FromFileError>>,
        affected_by_time_dilation: bool,
    ) {
        match handle {
            Either::Left(handle) => {
                self.store(tag_name, event_name, handle, affected_by_time_dilation)
            }
            Either::Right(handle) => {
                self.store(tag_name, event_name, handle, affected_by_time_dilation)
            }
        }
    }
}

impl Store<StaticSoundHandle> for Emitters {
    fn store(
        &mut self,
        tag_name: CName,
        event_name: CName,
        handle: StaticSoundHandle,
        affected_by_time_dilation: bool,
    ) {
        'outer: for mut slots in self.0.iter_mut() {
            for slot in slots.slots.iter_mut() {
                if slot.tag_name == Some(tag_name) {
                    slot.handles
                        .store_static(event_name, handle, affected_by_time_dilation);
                    break 'outer;
                }
            }
        }
    }
}

impl Store<StreamingSoundHandle<FromFileError>> for Emitters {
    fn store(
        &mut self,
        tag_name: CName,
        event_name: CName,
        handle: StreamingSoundHandle<FromFileError>,
        affected_by_time_dilation: bool,
    ) {
        'outer: for mut slots in self.0.iter_mut() {
            for slot in slots.slots.iter_mut() {
                if slot.tag_name == Some(tag_name) {
                    slot.handles
                        .store_stream(event_name, handle, affected_by_time_dilation);
                    break 'outer;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn collisions() {
        use std::hash::BuildHasher;
        use std::hash::Hash;
        use std::hash::Hasher;

        let mut hasher = ahash::RandomState::new().build_hasher();
        let no_distance: Option<crate::EmitterDistances> = None;
        no_distance.hash(&mut hasher);
        let hash_none = hasher.finish();
        assert_ne!(hash_none, 0);

        let some_distances = Some(crate::EmitterDistances {
            min_distance: 0.,
            max_distance: 0.,
        });
        some_distances.hash(&mut hasher);
        let hash_some_undefined = hasher.finish();
        assert_ne!(hash_some_undefined, 0);

        assert_ne!(hash_none, hash_some_undefined);
    }
}
