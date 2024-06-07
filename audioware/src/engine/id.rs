use std::hash::Hash;

use audioware_sys::interop::{
    entity::{find_entity_by_id, Entity},
    game::get_game_instance,
};
use red4ext_rs::types::{EntityId, Ref};
use snafu::OptionExt;
use snowflake::ProcessUniqueId;

use crate::bank::{Id, Key};

use super::error::CannotFindEntitySnafu;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct SoundEntityId(pub EntityId);

impl From<&EntityId> for SoundEntityId {
    fn from(value: &EntityId) -> Self {
        Self(value.clone())
    }
}

impl Hash for SoundEntityId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        u64::from(self.0.clone()).hash(state);
    }
}

impl PartialEq<EntityId> for SoundEntityId {
    fn eq(&self, other: &EntityId) -> bool {
        self.0.eq(other)
    }
}

impl TryFrom<&SoundEntityId> for Ref<Entity> {
    type Error = crate::engine::Error;

    fn try_from(value: &SoundEntityId) -> Result<Self, Self::Error> {
        find_entity_by_id(get_game_instance(), value.0.clone())
            .into_ref()
            .context(CannotFindEntitySnafu {
                entity_id: value.0.clone(),
            })
    }
}

#[derive(Debug, Clone, Eq)]
pub struct HandleId {
    pub id: ProcessUniqueId,
    pub key: Key,
    pub entity_id: Option<EntityId>,
}

impl HandleId {
    pub fn new(id: &Id, entity_id: Option<EntityId>) -> Self {
        Self {
            id: ProcessUniqueId::new(),
            key: AsRef::<Key>::as_ref(id).clone(),
            entity_id,
        }
    }
}

impl Hash for HandleId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for HandleId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
