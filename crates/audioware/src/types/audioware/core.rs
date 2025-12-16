use red4ext_rs::types::CName;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct EventName(CName);

impl TryFrom<CName> for EventName {
    type Error = ();

    fn try_from(value: CName) -> Result<Self, Self::Error> {
        if value == CName::undefined() {
            return Err(());
        }
        Ok(Self(value))
    }
}

impl std::fmt::Display for EventName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}
