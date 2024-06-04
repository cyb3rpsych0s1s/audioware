#![allow(unused_imports)]

mod dialog;
mod music;
mod sound;
mod voice;

pub use dialog::HookgameaudioeventsDialogLine;
pub use dialog::HookgameaudioeventsDialogLineEnd;
pub use dialog::HookgameaudioeventsStopDialogLine;

pub use music::HookgameaudioeventsMusicEvent;
pub use voice::HookgameaudioeventsVoiceEvent;
pub use voice::HookgameaudioeventsVoicePlayedEvent;

pub use sound::HookgameaudioeventsSound1;
// pub use sound::HookgameaudioeventsSound2;
pub use sound::HookgameaudioeventsSound3;
pub use sound::HookgameaudioeventsSound4;
pub use sound::HookgameaudioeventsSound5;
