use std::fmt::Debug;

use audioware_bank::Banks;
use dashmap::DashMap;
use kira::{
    manager::{backend::Backend, AudioManager, AudioManagerSettings},
    sound::{static_sound::StaticSoundHandle, streaming::StreamingSoundHandle, FromFileError},
};
use red4ext_rs::types::{CName, EntityId, Opt};
use snowflake::ProcessUniqueId;

use crate::error::{EngineError, Error};

pub mod queue;

#[derive(PartialEq)]
pub struct Emitter {
    pub id: EntityId,
    pub name: CName,
}
impl Emitter {
    pub fn new(id: Opt<EntityId>, name: Opt<CName>) -> Option<Self> {
        let id = id.into_option();
        let name = name.into_option();
        match (id, name) {
            (Some(id), Some(name)) => Some(Emitter { id, name }),
            _ => None,
        }
    }
}

pub struct Handle<T> {
    pub handle: T,
    pub event_name: CName,
    pub emitter: Option<Emitter>,
}
impl<T> Handle<T> {
    pub fn new(handle: T, event_name: CName, emitter: Option<Emitter>) -> Self {
        Self {
            handle,
            event_name,
            emitter,
        }
    }
}

pub struct Engine<B: Backend> {
    pub banks: Banks,
    pub manager: AudioManager<B>,
    pub statics: DashMap<ProcessUniqueId, Handle<StaticSoundHandle>>,
    pub streams: DashMap<ProcessUniqueId, Handle<StreamingSoundHandle<FromFileError>>>,
}

impl<B> Engine<B>
where
    B: Backend,
    <B as Backend>::Error: Debug,
{
    pub fn try_new(settings: AudioManagerSettings<B>) -> Result<Engine<B>, Error> {
        let banks = Banks::new();
        let manager = AudioManager::new(settings).map_err(|_| Error::Engine {
            source: EngineError::Manager {
                origin: "audio manager",
            },
        })?;
        Ok(Engine {
            banks,
            manager,
            statics: DashMap::new(),
            streams: DashMap::new(),
        })
    }
}
