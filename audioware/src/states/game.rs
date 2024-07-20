//! game state

use std::{
    convert::Infallible,
    sync::{atomic::AtomicU8, OnceLock},
};

use red4ext_rs::{log, NativeRepr, PluginOps};

use crate::Audioware;

use super::State;

/// retrieve [`State`]
fn state() -> &'static AtomicU8 {
    static INSTANCE: OnceLock<AtomicU8> = OnceLock::new();
    INSTANCE.get_or_init(|| AtomicU8::new(GameState::default() as u8))
}

impl State for GameState {
    type Value = Self;
    fn set(state: GameState) -> Self {
        let env = Audioware::env();
        let prev = self::state()
            .swap(state as u8, std::sync::atomic::Ordering::SeqCst)
            .try_into()
            .expect("game state is internally managed");
        if prev != state {
            log::info!(env, "game state: {prev} -> {state}");
        }
        prev
    }

    fn get() -> Self::Value {
        GameState::try_from(self::state().load(std::sync::atomic::Ordering::Relaxed))
            .expect("game state is internally managed")
    }
}

/// game lifecycle state
#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(i64)]
pub enum GameState {
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

unsafe impl NativeRepr for GameState {
    /// SAFETY: must match `Natives.reds`
    const NAME: &'static str = "Audioware.GameState";
}

impl TryFrom<u8> for GameState {
    type Error = Infallible;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v if GameState::Load as u8 == v => Ok(GameState::Load),
            v if GameState::Menu as u8 == v => Ok(GameState::Menu),
            v if GameState::Start as u8 == v => Ok(GameState::Start),
            v if GameState::InGame as u8 == v => Ok(GameState::InGame),
            v if GameState::InMenu as u8 == v => Ok(GameState::InMenu),
            v if GameState::InPause as u8 == v => Ok(GameState::InPause),
            v if GameState::End as u8 == v => Ok(GameState::End),
            v if GameState::Unload as u8 == v => Ok(GameState::Unload),
            _ => unreachable!("game state is internally managed"),
        }
    }
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameState::Load => write!(f, "plugin load"),
            GameState::Menu => write!(f, "main menu"),
            GameState::Start => write!(f, "game session start"),
            GameState::InGame => write!(f, "in-game"),
            GameState::InMenu => write!(f, "in-game menu"),
            GameState::InPause => write!(f, "in-game pause"),
            GameState::End => write!(f, "game session end"),
            GameState::Unload => write!(f, "plugin unload"),
        }
    }
}
