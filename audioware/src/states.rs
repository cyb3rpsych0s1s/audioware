mod game;
pub use game::GameState;
mod player;

#[allow(dead_code)]
pub trait State {
    type Value;
    fn set(value: Self::Value) -> Self::Value;
    fn get() -> Self::Value;
}
