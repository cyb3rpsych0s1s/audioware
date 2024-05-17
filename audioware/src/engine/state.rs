use std::sync::{atomic::AtomicU8, Mutex};

use once_cell::sync::OnceCell;
use red4ext_rs::conv::NativeRepr;

use super::effects::Preset;

fn state() -> &'static AtomicU8 {
    static INSTANCE: OnceCell<AtomicU8> = OnceCell::new();
    INSTANCE.get_or_init(|| AtomicU8::new(State::default() as u8))
}

fn preset() -> &'static Mutex<Preset> {
    static INSTANCE: OnceCell<Mutex<Preset>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

pub fn update(state: State) -> State {
    self::state()
        .swap(state as u8, std::sync::atomic::Ordering::SeqCst)
        .try_into()
        .unwrap()
}

#[allow(dead_code)]
pub fn load() -> State {
    self::state()
        .load(std::sync::atomic::Ordering::Relaxed)
        .try_into()
        .unwrap()
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

pub fn update_player_preset(value: Preset) -> anyhow::Result<()> {
    if let Ok(mut guard) = self::preset().try_lock() {
        *guard = value;
        return Ok(());
    }
    red4ext_rs::error!("lock contention");
    anyhow::bail!("lock contention")
}
