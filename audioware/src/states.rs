use std::sync::{Mutex, MutexGuard, OnceLock};

use audioware_manifest::PlayerGender;

use crate::error::InternalError;

mod game;
pub use game::*;

mod player;

pub trait State {
    type Value;
    fn set(value: Self::Value) -> Self::Value;
    fn get() -> Self::Value;
}

fn player_gender() -> &'static Mutex<Option<PlayerGender>> {
    static INSTANCE: OnceLock<Mutex<Option<PlayerGender>>> = OnceLock::new();
    INSTANCE.get_or_init(Default::default)
}

pub fn gender<'a>() -> Result<MutexGuard<'a, Option<PlayerGender>>, InternalError> {
    player_gender()
        .try_lock()
        .map_err(|_| InternalError::Contention {
            origin: "player gender",
        })
}
