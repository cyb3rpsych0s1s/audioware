//! Interop types used in audioware [.reds](https://github.com/cyb3rpsych0s1s/audioware/tree/main/audioware/reds).

mod core;
pub use core::*;
mod callback;
pub use callback::*;
mod easing;
pub use easing::*;
mod event;
pub use event::*;
mod settings;
pub use settings::*;
mod subtitles;
pub use subtitles::*;
mod tweens;
pub use tweens::*;
