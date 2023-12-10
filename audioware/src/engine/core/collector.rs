use std::{
    thread::JoinHandle,
    time::Duration, sync::OnceLock,
};

use lazy_static::lazy_static;

use super::{state::STATE, State, sounds};

lazy_static! {
    pub(super) static ref COLLECTOR: OnceLock<JoinHandle<()>> = OnceLock::default();
}

pub(super) fn setup() {
    let _ = COLLECTOR.set(std::thread::spawn(move || 'thread: loop {
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
    }));
}

pub(super) fn unpark() {
    COLLECTOR.get().unwrap().thread().unpark()
}

pub(super) fn collect() {}
