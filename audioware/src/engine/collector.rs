use std::{
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
    thread::JoinHandle,
    time::Duration,
};

use super::state::State;

pub(super) struct Collector(JoinHandle<()>);

impl Collector {
    pub(super) fn new(flag: Arc<AtomicU8>, callback: Arc<fn() -> ()>) -> Self {
        Self(std::thread::spawn(move || 'thread: loop {
            match flag.load(Ordering::Relaxed) {
                a if a == State::Load as u8 || a == State::Menu as u8 => {
                    std::thread::park();
                }
                a if a == State::InGame as u8 => {
                    (callback.clone())();
                    std::thread::sleep(Duration::from_millis(250));
                }
                _ => {
                    break 'thread;
                }
            }
        }))
    }
}
