use std::fmt::Debug;

use audioware_bank::Banks;
use kira::manager::{backend::Backend, AudioManager, AudioManagerSettings};

use crate::error::Error;

pub struct Engine<B: Backend> {
    pub banks: Banks,
    pub manager: AudioManager<B>,
}

impl<B> Engine<B>
where
    B: Backend,
    <B as Backend>::Error: Debug,
{
    pub fn try_new(settings: AudioManagerSettings<B>) -> Result<Engine<B>, Error> {
        let banks = Banks::new();
        let manager = AudioManager::new(settings).expect("instantiate audio manager");
        Ok(Engine { banks, manager })
    }
}
