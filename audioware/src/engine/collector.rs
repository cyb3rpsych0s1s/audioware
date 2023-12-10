use std::{sync::OnceLock, thread::JoinHandle, time::Duration};

use lazy_static::lazy_static;

use super::State;

lazy_static! {
    static ref COLLECTOR: OnceLock<JoinHandle<()>> = OnceLock::default();
}

pub(super) fn setup() {
    if let Err(_) = COLLECTOR.set(std::thread::spawn(move || 'thread: loop {
        match super::state::load() {
            State::Load | State::Menu | State::InMenu | State::InPause => {
                std::thread::park();
            }
            State::Start => {}
            State::InGame => {
                super::sounds::cleanup();
                std::thread::sleep(Duration::from_millis(250));
            }
            State::End => {
                super::sounds::terminate();
                std::thread::park();
            }
            State::Unload => {
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
