//! Interop types used in audioware [.reds](https://github.com/cyb3rpsych0s1s/audioware/tree/main/audioware/reds).

mod subtitles;
pub use subtitles::propagate_subtitles;

mod references;
pub use references::*;
mod easing;
pub use easing::*;
mod settings;
pub use settings::*;
mod tweens;
pub use tweens::*;
