use std::hash::Hash;

use red4ext_rs::types::EntityId;
use snowflake::ProcessUniqueId;

use crate::bank::{Id, Key};

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
