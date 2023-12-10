use std::{
    borrow::BorrowMut,
    collections::HashMap,
    sync::{Mutex, OnceLock},
    time::Duration,
};

use kira::{
    sound::{static_sound::StaticSoundHandle, PlaybackState},
    tween::Tween,
};
use lazy_static::lazy_static;

use crate::engine::id::SoundId;

lazy_static! {
    static ref SOUNDS_POOL: OnceLock<Mutex<HashMap<SoundId, StaticSoundHandle>>> =
        OnceLock::default();
}

const TERMINATE: Tween = Tween {
    start_time: kira::StartTime::Immediate,
    duration: Duration::from_secs(1),
    easing: kira::tween::Easing::Linear,
};

pub(super) fn cleanup() {
    if let Some(mut pool) = SOUNDS_POOL.get().and_then(|x| x.try_lock().ok()) {
        pool.retain(|_, v| v.state() != PlaybackState::Stopped);
    }
}

pub(super) fn terminate() {
    if let Some(mut pool) = SOUNDS_POOL.get().and_then(|x| x.try_lock().ok()) {
        // stop all sounds
        pool.values_mut().for_each(|v| {
            let _ = v.stop(TERMINATE);
        });
        // give some time for stop to kick in
        std::thread::sleep(TERMINATE.duration + Duration::from_millis(50));
        // then clear everything
        pool.clear();
    }
}
