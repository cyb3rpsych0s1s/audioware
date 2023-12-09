use std::sync::{atomic::AtomicU8, Arc};

use lazy_static::lazy_static;
use red4ext_rs::conv::NativeRepr;

lazy_static! {
    pub(super) static ref STATE: Arc<AtomicU8> = Arc::new(AtomicU8::new(State::Load as u8));
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(i64)]
pub enum State {
    /// load game
    #[default]
    Load = 0,
    /// game main menu
    Menu = 1,
    /// start game session
    Start = 2,
    /// roaming in-game
    InGame = 3,
    /// in menu in-game
    InMenu = 4,
    /// on pause in-game
    InPause = 5,
    /// end game session
    End = 6,
    /// unload game
    Unload = 7,
}

unsafe impl NativeRepr for State {
    const NAME: &'static str = "Audioware.EngineState";
}
