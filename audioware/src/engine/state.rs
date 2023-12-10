use std::sync::atomic::AtomicU8;

use anyhow::Ok;
use lazy_static::lazy_static;
use red4ext_rs::conv::NativeRepr;

lazy_static! {
    pub(super) static ref STATE: AtomicU8 = AtomicU8::new(State::default() as u8);
}

pub(super) fn update(state: State) {
    STATE.store(state as u8, std::sync::atomic::Ordering::SeqCst);
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
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

impl TryFrom<u8> for State {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v if State::Load as u8 == v => Ok(State::Load),
            v if State::Menu as u8 == v => Ok(State::Menu),
            v if State::Start as u8 == v => Ok(State::Start),
            v if State::InGame as u8 == v => Ok(State::InGame),
            v if State::InMenu as u8 == v => Ok(State::InMenu),
            v if State::InPause as u8 == v => Ok(State::InPause),
            v if State::End as u8 == v => Ok(State::End),
            v if State::Unload as u8 == v => Ok(State::Unload),
            _ => anyhow::bail!(format!("invalid State ({})", value)),
        }
    }
}
