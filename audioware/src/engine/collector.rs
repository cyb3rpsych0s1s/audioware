use std::{
    sync::{atomic::Ordering, Arc},
    thread::JoinHandle,
    time::Duration,
};

use super::state::{State, STATE};

pub(super) struct Collector(JoinHandle<()>);

impl Collector {
    pub(super) fn new(callback: Arc<fn() -> ()>) -> Self {
        Self(std::thread::spawn(move || 'thread: loop {
            match STATE.load(Ordering::Relaxed) {
                a if a == State::Load as u8 || a == State::Menu as u8 => {
                    std::thread::park();
                }
                a if a == State::Start as u8 => {}
                a if a == State::InGame as u8
                    || a == State::InMenu as u8
                    || a == State::InPause as u8 =>
                {
                    (callback.clone())();
                    std::thread::sleep(Duration::from_millis(250));
                }
                a if a == State::End as u8 => {
                    (callback.clone())();
                    std::thread::park();
                }
                _ => {
                    break 'thread;
                }
            }
        }))
    }
    pub(crate) fn unpark(&self) {
        self.0.thread().unpark();
    }
}
