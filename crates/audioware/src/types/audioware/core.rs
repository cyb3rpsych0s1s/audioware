use red4ext_rs::{NativeRepr, types::CName};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct EventName(CName);

unsafe impl NativeRepr for EventName {
    const NAME: &'static str = CName::NAME;
}

impl TryFrom<CName> for EventName {
    type Error = ();

    fn try_from(value: CName) -> Result<Self, Self::Error> {
        if value == CName::undefined() {
            return Err(());
        }
        Ok(Self(value))
    }
}
