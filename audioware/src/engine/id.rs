use std::hash::Hash;

use audioware_bank::{Id, Key};
use red4ext_rs::types::EntityId;
use snowflake::ProcessUniqueId;

/// Represents a currently playing audio resource ID.
#[derive(Debug, Clone)]
pub struct HandleId {
    id: ProcessUniqueId,
    key: Key,
    entity_id: Option<EntityId>,
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

/// Represents a currently registered spatial audio scene
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EmitterId(EntityId);
