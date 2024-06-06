use red4ext_rs::types::CName;
use red4ext_rs::types::EntityId;

mod address;
mod audiosystem;
mod event;
pub(super) use audiosystem::*;
pub(super) use event::*;

/// check if `T` is not [`Eq`]uals to [`Default`],
/// otherwise returns [`None`]
/// 
/// This is useful for parameters that go over the bridge:
/// they have to be defined in FFI,
/// but could be defined as `opt` in Redscript.
pub trait Maybe
where
    Self: Default + PartialEq,
{
    fn maybe(&self) -> Option<&Self> {
        if self == &Self::default() {
            return None;
        }
        Some(self)
    }
}

impl Maybe for CName {}
impl Maybe for EntityId {}
