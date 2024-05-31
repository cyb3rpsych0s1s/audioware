use red4ext_rs::{
    conv::ClassType,
    types::{IScriptable, Ref},
};

use crate::error::{DowncastError, UpcastError};

pub mod angles;
pub mod audio;
pub mod cruid;
pub mod entity;
pub mod event;
pub mod game;
pub mod gender;
pub mod hash;
pub mod icomponent;
pub mod iscriptable;
pub mod locale;
pub mod localization;
pub mod quaternion;
pub mod reflection;
pub mod vector4;

pub enum Frame {
    Initial(),
}

pub trait AsParent {
    type Parent;
    fn as_parent(&self) -> Result<Ref<Self::Parent>, UpcastError>;
}

impl<T> AsParent for Ref<T>
where
    T: ClassType,
{
    type Parent = T::BaseClass;

    fn as_parent(&self) -> Result<Ref<Self::Parent>, UpcastError> {
        if std::any::type_name::<Self>() == std::any::type_name::<IScriptable>() {
            return Err(UpcastError::NoParentClass);
        }
        Ok(Ref::upcast(self.clone()))
    }
}

pub trait SafeDowncast<T: ClassType> {
    fn downcast(&self) -> Result<&Ref<T>, DowncastError>;
    fn maybe_downcast(&self) -> Option<&Ref<T>> {
        self.downcast().ok()
    }
}

#[macro_export]
macro_rules! impl_safe_downcast {
    ($this:ident, $that:ident) => {
        impl $crate::interop::SafeDowncast<$that> for Ref<$this> {
            fn downcast(self: &Self) -> Result<&Ref<$that>, $crate::error::DowncastError> {
                red4ext_rs::warn!("enter safe downcast");
                if Self::is_a(
                    self.clone(),
                    red4ext_rs::types::CName::new($that::NATIVE_NAME),
                ) {
                    // SAFETY: already checked RTTI above
                    red4ext_rs::warn!("before safe downcast");
                    let reference: &Ref<$that> = unsafe { std::mem::transmute(self) };
                    red4ext_rs::warn!("after safe downcast");
                    return Ok(reference);
                }
                Err($crate::error::DowncastError::Invalid {
                    current: std::any::type_name::<Self>(),
                    class: $that::NATIVE_NAME,
                })
            }
        }
    };
}
