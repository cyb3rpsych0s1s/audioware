use red4ext_rs::types::CName;
use red4ext_rs::types::EntityId;

mod address;
mod audiosystem;
mod event;
pub(super) use audiosystem::*;
pub(super) use event::*;

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