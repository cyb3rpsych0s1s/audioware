use std::hash::Hash;

use audioware_bank::{Id, Key};
use red4ext_rs::types::{CName, EntityId};
use snowflake::ProcessUniqueId;

/// Represents a currently playing audio resource ID.
#[derive(Debug, Clone)]
pub struct HandleId {
    id: ProcessUniqueId,
    key: Key,
    entity_id: Option<EntityId>,
    emitter_name: Option<CName>,
}

impl HandleId {
    pub fn new(id: &Id, entity_id: Option<EntityId>, emitter_name: Option<CName>) -> Self {
        Self {
            id: ProcessUniqueId::new(),
            key: AsRef::<Key>::as_ref(id).clone(),
            entity_id,
            emitter_name,
        }
    }
    pub fn event_name(&self) -> &CName {
        self.key.as_ref()
    }
    pub fn entity_id(&self) -> Option<&EntityId> {
        self.entity_id.as_ref()
    }
    pub fn emitter_name(&self) -> Option<&CName> {
        self.emitter_name.as_ref()
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

impl Eq for HandleId {}

/// Represents a currently registered spatial audio scene
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmitterId {
    id: EntityId,
    name: Option<CName>,
}

impl EmitterId {
    pub fn new(entity_id: EntityId, emitter_name: Option<CName>) -> Self {
        Self {
            id: entity_id,
            name: emitter_name,
        }
    }
    pub fn entity_id(&self) -> &EntityId {
        &self.id
    }
}
