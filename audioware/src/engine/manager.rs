use std::{sync::{Arc, Mutex, OnceLock, MutexGuard}, borrow::BorrowMut};

use kira::{manager::{backend::DefaultBackend, AudioManager}, sound::{SoundData, Sound}};
use lazy_static::lazy_static;

use super::wrapper::OnceWrapper;

lazy_static! {
    static ref MANAGER: OnceWrapper<AudioManager<DefaultBackend>> = OnceWrapper::default();
}

pub(super) struct Manager;

impl Manager {
    pub(super) fn play<T: SoundData>(&mut self, sound: T) -> anyhow::Result<T::Handle> {
        MANAGER.try_call::<T::Handle>(move |mut guard: MutexGuard<'_, AudioManager>| {
            match guard.play(sound) {
                Ok(handle) => Ok(handle),
                Err(_) => anyhow::bail!("unable to play song"),
            }
        })
    }
}
