mod game;
pub use game::GameState;
mod player;

pub trait State {
    type Value;
    fn swap(value: Self::Value) -> Self::Value;
    fn set(value: Self::Value) {
        let _ = Self::swap(value);
    }
    fn get() -> Self::Value;
}
