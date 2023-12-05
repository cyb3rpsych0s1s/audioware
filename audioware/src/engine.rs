use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
};

use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    track::TrackBuilder,
};
use lazy_static::lazy_static;

use crate::Audioware;

lazy_static! {
    static ref AUDIO: Arc<Mutex<Audioware>> = Arc::new(Mutex::new(Audioware::default()));
}

impl Audioware {
    fn setup() -> anyhow::Result<AudioManager> {
        let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
        let vocal = TrackBuilder::default();
        let mental = TrackBuilder::default();
        let emission = TrackBuilder::default();
        manager.add_sub_track(vocal)?;
        manager.add_sub_track(mental)?;
        manager.add_sub_track(emission)?;
        Ok(manager)
    }
    pub(crate) fn create() -> anyhow::Result<()> {
        match Self::setup() {
            Ok(manager) => match AUDIO.clone().borrow_mut().try_lock() {
                Ok(mut guard) => {
                    *guard = Self(Some(manager));
                    Ok(())
                }
                Err(_) => anyhow::bail!("unable to store audioware's handle"),
            },
            Err(_) => anyhow::bail!("unable to setup audioware's audio engine"),
        }
    }
    pub(crate) fn destroy() -> anyhow::Result<()> {
        match AUDIO.clone().borrow_mut().try_lock() {
            Ok(mut guard) => {
                *guard = Self(None);
                Ok(())
            }
            Err(_) => anyhow::bail!("unable to destroy audioware"),
        }
    }
}
