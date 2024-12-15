use std::{collections::HashSet, sync::LazyLock};

use dashmap::{iter::IterMut, mapref::one::RefMut, DashMap};
use parking_lot::RwLock;
use red4ext_rs::types::{CName, EntityId};

use super::emitter::Emitter;

#[allow(clippy::type_complexity)]
pub static EMITTERS: LazyLock<RwLock<HashSet<(EntityId, Option<CName>)>>> =
    LazyLock::new(|| RwLock::new(HashSet::new()));

pub struct Emitters(pub DashMap<EntityId, Emitter>);

impl Emitters {
    pub fn with_capacity(capacity: usize) -> Self {
        *EMITTERS.write() = HashSet::with_capacity(capacity);
        Self(DashMap::with_capacity(capacity))
    }
    pub fn exists(&self, entity_id: &EntityId) -> bool {
        self.0.contains_key(entity_id)
    }
    pub fn get_mut(&mut self, entity_id: &EntityId) -> Option<RefMut<'_, EntityId, Emitter>> {
        self.0.get_mut(entity_id)
    }
    pub fn get_mut_by_name(
        &mut self,
        entity_id: &EntityId,
        emitter_name: &Option<CName>,
    ) -> Option<RefMut<'_, EntityId, Emitter>> {
        if let Some(emitter) = self.0.get_mut(entity_id) {
            if emitter.names.contains(emitter_name) {
                return Some(emitter);
            }
        }
        None
    }
    pub fn insert(
        &mut self,
        entity_id: EntityId,
        emitter_name: Option<CName>,
        value: Emitter,
    ) -> Option<Emitter> {
        let inserted = self.0.insert(entity_id, value);
        if inserted.is_none() {
            EMITTERS.write().insert((entity_id, emitter_name));
        }
        inserted
    }
    pub fn remove(&mut self, entity_id: EntityId) -> bool {
        let mut removed = false;
        self.0.retain(|k, _| {
            if *k == entity_id {
                removed = true;
                false
            } else {
                true
            }
        });
        if !removed {
            return false;
        }
        EMITTERS.write().retain(|(id, _)| *id != entity_id);
        true
    }
    pub fn iter_mut(&mut self) -> IterMut<EntityId, Emitter> {
        self.0.iter_mut()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&EntityId, &mut Emitter) -> bool,
    {
        self.0.retain(f)
    }
    pub fn clear(&mut self) {
        EMITTERS.write().clear();
        self.0.clear();
    }
}

impl Drop for Emitters {
    fn drop(&mut self) {
        EMITTERS.write().clear();
    }
}
