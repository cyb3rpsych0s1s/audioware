use std::{collections::HashSet, num::NonZero, sync::LazyLock};

use dashmap::{
    mapref::{multiple::RefMutMulti, one::RefMut},
    DashMap,
};
use kira::{
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, FromFileError},
    spatial::emitter::{EmitterHandle, EmitterId, EmitterSettings},
    tween::Tween,
};
use mods::EmitterMod;
use parking_lot::RwLock;
use red4ext_rs::types::{CName, EntityId};
use slots::{EmitterFootprint, EmitterSlot, EmitterSlots};

use crate::{
    engine::tweens::IMMEDIATELY,
    error::{Error, SceneError},
    utils::lifecycle,
    Vector4,
};

mod emitter;
mod handles;
mod mods;
mod slots;

pub use emitter::Emitter;

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
        entity_id: EntityId,
        tag_name: CName,
        emitter_name: Option<CName>,
        settings: Option<(EmitterSettings, NonZero<u64>)>,
        handle: EmitterHandle,
        dilation: Option<f32>,
        last_known_position: Vector4,
        busy: bool,
    ) -> Result<(), Error> {
        self.0.insert(
            entity_id,
            EmitterSlots::new(dilation, busy, last_known_position),
        );
        self.0
            .get_mut(&entity_id)
            .expect("previously inserted")
            .insert(
                EmitterFootprint::new(settings),
                EmitterSlot::new(handle, tag_name, emitter_name),
            );
        EMITTERS.write().insert((entity_id, tag_name));
        lifecycle!(
            "added emitter {entity_id} with tag name {}",
            tag_name.as_str()
        );
        Ok(())
    }
    pub fn pair_emitter(
        &mut self,
        entity_id: EntityId,
        tag_name: CName,
        emitter_name: Option<CName>,
        settings: Option<&(EmitterSettings, NonZero<u64>)>,
    ) -> Result<bool, Error> {
        // check whether the emitter has already been registered for this tag
        if self.exists_tag(&entity_id, &tag_name) {
            return Err(Error::Scene {
                source: SceneError::DuplicateEmitter {
                    entity_id,
                    tag_name,
                },
            });
        }

        // check whether a previously registered emitter with same settings can be reused
        if let Some(entry) = self.0.get_mut(&entity_id) {
            if let Some(entry) = entry.get(settings.map(|(_, x)| *x)) {
                entry.mods.insert(tag_name, EmitterMod::new(emitter_name));
                EMITTERS.write().insert((entity_id, tag_name));
                lifecycle!(
                    "emitter already exists, paired {} [{entity_id}]",
                    tag_name.as_str()
                );
                return Ok(true);
            }
        }
        Ok(false)
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
        removed
    }
    pub fn on_emitter_dies(&mut self, entity_id: &EntityId) {
        self.0.retain(|k, v| {
            if k == entity_id {
                let mut retain = false;
                v.slots.retain(|ki, vi| {
                    if ki.persist_until_sounds_finish {
                        retain = true;
                        true
                    } else {
                        vi.stop_emitters(IMMEDIATELY);
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
        if let Some(slots) = self.0.get_mut(&entity_id) {
            for ref mut slot in slots.slots.iter_mut() {
                if !slot.key().persist_until_sounds_finish {
                    slot.value_mut().stop_emitters(tween);
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
    pub fn stop_emitters(&mut self, tween: Tween) {
        self.0.iter_mut().for_each(|mut x| {
            x.value_mut().stop_emitters(tween);
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
    pub fn emitter_destination(
        &self,
        entity_id: &EntityId,
        tag_name: &CName,
    ) -> Option<(EmitterId, Option<CName>)> {
        self.0
            .iter()
            .find(|x| x.key() == entity_id)
            .and_then(|x| x.emitter_destination(tag_name))
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
        emitter_id: EmitterId,
        event_name: CName,
        handle: T,
        affected_by_time_dilation: bool,
    );
}

impl Store<StaticSoundHandle> for Emitters {
    fn store(
        &mut self,
        tag_name: CName,
        emitter_id: EmitterId,
        event_name: CName,
        handle: StaticSoundHandle,
        affected_by_time_dilation: bool,
    ) {
        'outer: for ref mut slots in self.0.iter_mut() {
            for ref mut slot in slots.slots.iter_mut() {
                if emitter_id == slot.handle.id() {
                    if let Some(mut r#mod) = slot.value_mut().mods.get_mut(&tag_name) {
                        r#mod
                            .handles
                            .store_static(event_name, handle, affected_by_time_dilation);
                    }
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
        emitter_id: EmitterId,
        event_name: CName,
        handle: StreamingSoundHandle<FromFileError>,
        affected_by_time_dilation: bool,
    ) {
        'outer: for ref mut slots in self.0.iter_mut() {
            for ref mut slot in slots.slots.iter_mut() {
                if emitter_id == slot.handle.id() {
                    if let Some(mut r#mod) = slot.value_mut().mods.get_mut(&tag_name) {
                        r#mod
                            .handles
                            .store_stream(event_name, handle, affected_by_time_dilation);
                    }
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
