mod game;
pub use game::GameState;

mod player;

pub trait State {
    type Value: PartialEq + Clone;
    fn swap(value: Self::Value) -> Self::Value;
    fn set(value: Self::Value) {
        Self::swap(value);
    }
    fn get() -> Self::Value;
}

pub trait ToggleState: State {
    fn set_and_toggle(value: Self::Value) {
        let prior = Self::swap(value.clone());
        Self::toggle(prior, value);
    }
    fn toggle(before: Self::Value, after: Self::Value);
}
