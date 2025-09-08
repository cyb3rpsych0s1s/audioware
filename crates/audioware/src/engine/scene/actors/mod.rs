use std::{collections::HashSet, sync::LazyLock};

use dashmap::{DashMap, mapref::one::RefMut};
use kira::sound::FromFileError;
use parking_lot::RwLock;
use red4ext_rs::types::{Cruid, EntityId};

use crate::engine::{scene::actors::slot::ActorSlot, traits::DualHandles};

pub mod slot;

#[allow(clippy::type_complexity)]
pub static ACTORS: LazyLock<RwLock<HashSet<EntityId>>> =
    LazyLock::new(|| RwLock::new(HashSet::new()));

pub struct Actors {
    pub v: DualHandles<Cruid, (), FromFileError>,
    pub holocall: DualHandles<Cruid, (), FromFileError>,
    pub emitters: DashMap<EntityId, ActorSlot>,
}

impl Actors {
    pub fn with_capacity(capacity: usize) -> Self {
        *ACTORS.write() = HashSet::with_capacity(capacity);
        Self {
            v: Default::default(),
            holocall: Default::default(),
            emitters: Default::default(),
        }
    }
    pub fn exists(&self, entity_id: &EntityId) -> bool {
        self.emitters.contains_key(entity_id)
    }
    pub fn get_emitter_track_mut<'a>(
        &'a self,
        entity_id: &EntityId,
    ) -> Option<RefMut<'a, EntityId, ActorSlot>> {
        self.emitters.get_mut(entity_id)
    }
}
