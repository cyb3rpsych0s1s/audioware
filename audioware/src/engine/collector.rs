use std::{sync::OnceLock, thread::JoinHandle, time::Duration};

use lazy_static::lazy_static;

use super::{sounds, state::STATE, State};

lazy_static! {
    pub(super) static ref COLLECTOR: OnceLock<JoinHandle<()>> = OnceLock::default();
}

pub(super) fn setup() {
    if let Err(_) = COLLECTOR.set(std::thread::spawn(move || 'thread: loop {
        match STATE.load(std::sync::atomic::Ordering::Relaxed) {
            a if a == State::Load as u8 || a == State::Menu as u8 => {
                std::thread::park();
            }
            a if a == State::Start as u8 => {}
            a if a == State::InGame as u8
                || a == State::InMenu as u8
                || a == State::InPause as u8 =>
            {
                sounds::cleanup();
                std::thread::sleep(Duration::from_millis(250));
            }
            a if a == State::End as u8 => {
                sounds::terminate();
                std::thread::park();
            }
            _ => {
                break 'thread;
            }
        }
    })) {
        red4ext_rs::error!("error on initializing collector");
    }
}

pub(super) fn unpark() {
    COLLECTOR.get().unwrap().thread().unpark()
}

pub(super) fn collect() {}
