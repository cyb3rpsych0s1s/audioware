use std::{collections::HashSet, num::NonZero, sync::LazyLock};

use dashmap::{
    iter::IterMut,
    mapref::{multiple::RefMutMulti, one::RefMut},
    DashMap,
};
use parking_lot::RwLock;
use red4ext_rs::types::{CName, EntityId, Ref};

use crate::utils::fails;

use super::emitter::Emitter;

#[allow(clippy::type_complexity)]
pub static EMITTERS: LazyLock<RwLock<HashSet<(EntityId, CName)>>> =
    LazyLock::new(|| RwLock::new(HashSet::new()));

pub struct Emitters(pub DashMap<EmitterKey, Emitter, ahash::RandomState>);

impl Emitters {
    pub fn with_capacity(capacity: usize) -> Self {
        *EMITTERS.write() = HashSet::with_capacity(capacity);
        Self(DashMap::with_capacity_and_hasher(
            capacity,
            ahash::RandomState::new(),
        ))
    }
    pub fn exists(&self, entity_id: &EntityId, tag_name: &CName) -> bool {
        self.0
            .iter()
            .any(|x| x.key().entity_id == *entity_id && x.value().sharers.contains(tag_name))
    }
    pub fn get_mut(&mut self, key: &EmitterKey) -> Option<RefMut<'_, EmitterKey, Emitter>> {
        self.0.get_mut(key)
    }
    pub fn get_mut_by_name(
        &mut self,
        entity_id: &EntityId,
        tag_name: &CName,
    ) -> Option<RefMutMulti<'_, EmitterKey, Emitter>> {
        self.0
            .iter_mut()
            .find(|x| x.key().entity_id == *entity_id && x.value().sharers.contains(tag_name))
    }
    pub fn insert(&mut self, key: EmitterKey, tag_name: CName, value: Emitter) -> Option<Emitter> {
        let EmitterKey { entity_id, .. } = key;
        let inserted = self.0.insert(key, value);
        if inserted.is_none() {
            EMITTERS.write().insert((entity_id, tag_name));
        }
        inserted
    }
    pub fn remove(&mut self, entity_id: EntityId) -> bool {
        let mut removed = false;
        self.0.retain(|k, _| {
            if k.entity_id == entity_id {
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
    pub fn iter_mut(&mut self) -> IterMut<EmitterKey, Emitter, ahash::RandomState> {
        self.0.iter_mut()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&EmitterKey, &mut Emitter) -> bool,
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

#[derive(Debug, PartialEq, Eq)]
pub struct EmitterKey {
    pub entity_id: EntityId,
    pub settings_hash: Option<NonZero<u64>>,
}

impl std::hash::Hash for EmitterKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.entity_id.hash(state);
        self.settings_hash.hash(state);
    }
}

impl From<(&EntityId, &Ref<crate::EmitterSettings>)> for EmitterKey {
    fn from(value: (&EntityId, &Ref<crate::EmitterSettings>)) -> Self {
        let settings_hash = if value.1.is_null() {
            None::<NonZero<u64>>
        } else {
            match ahash::RandomState::new().hash_one(unsafe { value.1.fields() }) {
                0 => {
                    fails!("emitter settings hash should not be 0");
                    None
                }
                hash => Some(NonZero::new(hash).unwrap()),
            }
        };
        Self {
            entity_id: *value.0,
            settings_hash,
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
