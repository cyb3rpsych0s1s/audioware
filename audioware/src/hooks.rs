#[rustfmt::skip]
mod offsets {
    pub(super) const PLAY: u32              = 0xCDB11D0E; // 0x140974F58
    pub(super) const PLAY_ON_EMITTER: u32   = 0x48D20A5;  // 0x141C01EF0
    pub(super) const STOP: u32              = 0xD2781D1E; // 0x1424503F8
    pub(super) const SWITCH: u32            = 0x15081DEA; // 0x140291688
}

pub mod play;
pub mod play_on_emitter;
pub mod stop;
pub mod switch;
