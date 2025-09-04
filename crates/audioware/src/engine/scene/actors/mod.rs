use std::{collections::HashSet, sync::LazyLock};

use audioware_bank::{BankData, Banks, SceneId};
use dashmap::DashMap;
use either::Either;
use kira::Tween;
use parking_lot::RwLock;
use red4ext_rs::types::{Cruid, EntityId};

use crate::{
    Vector4,
    engine::{scene::actors::slot::ActorSlot, tracks::Spatial, traits::stop::StopBy},
    error::{EngineError, Error, SceneError},
};

mod slot;

#[allow(clippy::type_complexity)]
pub static ACTORS: LazyLock<RwLock<HashSet<EntityId>>> =
    LazyLock::new(|| RwLock::new(HashSet::new()));

pub struct Actors(DashMap<EntityId, ActorSlot>);

impl Actors {
    pub fn with_capacity(capacity: usize) -> Self {
        *ACTORS.write() = HashSet::with_capacity(capacity);
        Self(Default::default())
    }
    pub fn exists(&self, entity_id: &EntityId) -> bool {
        self.0.contains_key(entity_id)
    }
    pub fn add_actor(
        &mut self,
        handle: Spatial,
        entity_id: EntityId,
        last_known_position: Vector4,
    ) -> Result<(), Error> {
        self.0.insert(
            entity_id,
            ActorSlot {
                handle,
                handles: Default::default(),
                last_known_position,
            },
        );
        Ok(())
    }
    pub fn play_on_actor(
        &mut self,
        key: &SceneId,
        banks: &Banks,
        event_name: Cruid,
        entity_id: EntityId,
    ) -> Result<(), Error> {
        let Some(mut slot) = self.0.get_mut(&entity_id) else {
            return Err(SceneError::MissingActor { entity_id }.into());
        };
        let data = banks.data(key);
        slot.play_and_store(event_name, data).map_err(|e| match e {
            Either::Left(e) => Error::Engine {
                source: EngineError::Sound { source: e },
            },
            Either::Right(e) => Error::Engine {
                source: EngineError::FromFile { source: e },
            },
        })
    }

    pub fn stop_on_actor(&mut self, cruid: &Cruid, tween: Tween) -> Result<(), Error> {
        for mut slot in self.0.iter_mut() {
            slot.handles.stop_by(cruid, tween);
        }
        Ok(())
    }
}
