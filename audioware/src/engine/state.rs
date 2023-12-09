#[repr(u8)]
pub(super) enum State {
    Load = 0,
    Menu = 1,
    InGame = 2,
    Unload = 3,
}
