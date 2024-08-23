//! Game states.

mod game;
pub use game::GameState;

mod player;

/// Any [value][State::Value]-based state.
pub trait State {
    type Value: PartialEq + Clone;
    fn swap(value: Self::Value) -> Self::Value;
    fn set(value: Self::Value) {
        Self::swap(value);
    }
    fn get() -> Self::Value;
}

/// Any toggleable [State]
/// which conditionally trigger additional logic.
pub trait ToggleState: State {
    /// Set new value, calling toggle with prior and new value.
    fn set_and_toggle(value: Self::Value) {
        let prior = Self::swap(value.clone());
        Self::toggle(prior, value);
    }
    /// Trigger conditional logic when [State] changes.
    fn toggle(before: Self::Value, after: Self::Value);
}
