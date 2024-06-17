#[macro_export]
macro_rules! typed_cname {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        #[repr(transparent)]
        pub struct $name(CName);

        impl Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                u64::from(self.0.clone()).hash(state);
            }
        }

        impl PartialEq<CName> for $name {
            fn eq(&self, other: &CName) -> bool {
                self.0.eq(other)
            }
        }

        impl PartialEq<$name> for CName {
            fn eq(&self, other: &$name) -> bool {
                self.eq(&other.0)
            }
        }

        impl FromRepr for $name {
            type Repr = CName;
            fn from_repr(repr: Self::Repr) -> Self {
                Self(Self::Repr::from_repr(repr))
            }
        }
    };
}
pub use typed_cname;
