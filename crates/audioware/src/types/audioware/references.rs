use std::fmt;

use red4ext_rs::{types::Ref, ClassKind, ScriptClass};

#[derive(Clone, Default)]
#[repr(transparent)]
pub struct RedRef<T: ScriptClass>(Ref<T>);

unsafe impl<T: ScriptClass> ScriptClass for RedRef<T>
where
    <T as ScriptClass>::Kind: ClassKind<RedRef<T>>,
{
    type Kind = T::Kind;
    const NAME: &'static str = T::NAME;
}

impl<T: ScriptClass> fmt::Debug for RedRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!("Ref<{}>", T::NAME);
        f.debug_tuple(&s)
            .field(&format_args!(
                "inner: {}",
                if self.0.is_null() {
                    "undefined"
                } else {
                    "defined"
                }
            ))
            .finish()
    }
}

impl<T: ScriptClass> From<Ref<T>> for RedRef<T> {
    fn from(value: Ref<T>) -> Self {
        Self(value)
    }
}

impl<T: ScriptClass> From<RedRef<T>> for Ref<T> {
    fn from(value: RedRef<T>) -> Self {
        value.0
    }
}
