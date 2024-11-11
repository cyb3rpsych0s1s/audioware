use std::fmt::Debug;

use audioware_bank::{BankData, Banks};
use audioware_manifest::{PlayerGender, SpokenLocale};
use either::Either;
use handles::{Emitter, Handles};
use kira::manager::{backend::Backend, AudioManager, AudioManagerSettings};
use red4ext_rs::types::{CName, EntityId, Opt, Ref};

use crate::{
    error::{EngineError, Error},
    Tween,
};

mod handles;
pub mod queue;

pub struct Engine<B: Backend> {
    pub banks: Banks,
    pub manager: AudioManager<B>,
    pub handles: Handles,
}

impl<B> Engine<B>
where
    B: Backend,
    <B as Backend>::Error: Debug,
{
    pub fn try_new(settings: AudioManagerSettings<B>) -> Result<Engine<B>, Error> {
        let banks = Banks::new();
        let capacity = settings.capacities.sound_capacity as usize;
        let manager = AudioManager::new(settings).map_err(|_| Error::Engine {
            source: EngineError::Manager {
                origin: "audio manager",
            },
        })?;
        Ok(Engine {
            banks,
            manager,
            handles: Handles::with_capacity(capacity),
        })
    }

    pub fn play(
        &mut self,
        event_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        spoken: SpokenLocale,
        gender: Option<PlayerGender>,
    ) {
        if let Ok(key) = self.banks.try_get(&event_name, &spoken, gender.as_ref()) {
            let data = self.banks.data(key);
            let emitter = Emitter::new(entity_id, emitter_name);
            match data {
                Either::Left(data) => {
                    if let Ok(handle) = self.manager.play(data) {
                        self.handles.store_static(handle, event_name, emitter);
                    }
                }
                Either::Right(data) => {
                    if let Ok(handle) = self.manager.play(data) {
                        self.handles.store_stream(handle, event_name, emitter);
                    }
                }
            }
        }
    }

    pub fn stop(
        &mut self,
        event_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        tween: Ref<Tween>,
    ) {
        if self.banks.exists(&event_name) {
            self.handles
                .stop(event_name, Emitter::new(entity_id, emitter_name), tween);
        }
    }

    pub fn pause(&mut self) {
        self.handles.pause();
    }

    pub fn resume(&mut self) {
        self.handles.resume();
    }

    pub fn reclaim(&mut self) {
        self.handles.reclaim();
    }
}
