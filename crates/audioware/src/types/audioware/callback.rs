use std::cell::Cell;

use red4ext_rs::{
    NativeRepr, ScriptClass,
    class_kind::Native,
    types::{IScriptable, Ref},
};

use crate::{
    AudioEventCallbackEntityTarget, AudioEventCallbackEventTarget, CallbackSystemTarget,
    abi::callback::Callback, engine::queue,
};

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct AudioEventCallbackSystem {
    base: IScriptable,
}

unsafe impl ScriptClass for AudioEventCallbackSystem {
    type Kind = Native;
    const NAME: &'static str = "Audioware.AudioEventCallbackSystem";
}

impl AsRef<IScriptable> for AudioEventCallbackSystem {
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}

pub trait Handler {
    fn unregister(&self);
    fn is_registered(&self) -> bool;
    fn add_target(&self, value: Ref<CallbackSystemTarget>) -> Ref<Self>
    where
        Self: Sized + ScriptClass;
    fn remove_target(&self, value: Ref<CallbackSystemTarget>) -> Ref<Self>
    where
        Self: Sized + ScriptClass;
    fn set_lifetime(&self, value: AudioEventCallbackLifetime) -> Ref<Self>
    where
        Self: Sized + ScriptClass;
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum AudioEventCallbackLifetime {
    #[default]
    Session = 0,
    Forever = 1,
}

unsafe impl NativeRepr for AudioEventCallbackLifetime {
    const NAME: &'static str = "Audioware.AudioEventCallbackLifetime";
}

impl std::fmt::Display for AudioEventCallbackLifetime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Forever => "Forever",
                Self::Session => "Session",
            }
        )
    }
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct AudioEventCallbackHandler {
    base: IScriptable,
    pub(crate) id: Cell<u32>,
}

unsafe impl ScriptClass for AudioEventCallbackHandler {
    type Kind = Native;
    const NAME: &'static str = "Audioware.AudioEventCallbackHandler";
}

impl AsRef<IScriptable> for AudioEventCallbackHandler {
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}

impl Handler for AudioEventCallbackHandler {
    fn unregister(&self) {
        queue::forward(Callback::Unregister { id: self.id.get() });
    }

    fn is_registered(&self) -> bool {
        AudioEventCallbackSystem::is_registered(self.id.get())
    }

    fn add_target(&self, value: Ref<CallbackSystemTarget>) -> Ref<Self>
    where
        Self: Sized + ScriptClass,
    {
        let add = true;
        if !value.is_null() {
            if let Some(value) = value.clone().cast::<AudioEventCallbackEntityTarget>() {
                if let Some(value) = unsafe { value.fields() }
                    && let Some(value) = value.value.as_ref()
                {
                    match value {
                        super::EntityTargetInner::Id(entity_id) => {
                            queue::forward(Callback::Filter {
                                id: self.id.get(),
                                target: crate::AnyTarget::Id(*entity_id),
                                add,
                            });
                        }
                        super::EntityTargetInner::EmitterName(cname) => {
                            queue::forward(Callback::Filter {
                                id: self.id.get(),
                                target: crate::AnyTarget::EmitterName(*cname),
                                add,
                            })
                        }
                    };
                }
            } else if let Some(value) = value.cast::<AudioEventCallbackEventTarget>()
                && let Some(value) = unsafe { value.fields() }
                && let Some(value) = value.value.as_ref()
            {
                match value {
                    super::EventTargetInner::ActionType(ty) => {
                        queue::forward(Callback::Filter {
                            id: self.id.get(),
                            target: crate::AnyTarget::Type(*ty),
                            add,
                        });
                    }
                };
            }
        }
        Ref::new_with(|x: &mut Self| {
            x.id.set(self.id.get());
        })
        .unwrap_or_default()
    }

    fn remove_target(&self, value: Ref<CallbackSystemTarget>) -> Ref<Self>
    where
        Self: Sized + ScriptClass,
    {
        let add = false;
        if !value.is_null() {
            if let Some(value) = value.clone().cast::<AudioEventCallbackEntityTarget>() {
                if let Some(value) = unsafe { value.fields() }
                    && let Some(value) = value.value.as_ref()
                {
                    match value {
                        super::EntityTargetInner::Id(entity_id) => {
                            queue::forward(Callback::Filter {
                                id: self.id.get(),
                                target: crate::AnyTarget::Id(*entity_id),
                                add,
                            });
                        }
                        super::EntityTargetInner::EmitterName(cname) => {
                            queue::forward(Callback::Filter {
                                id: self.id.get(),
                                target: crate::AnyTarget::EmitterName(*cname),
                                add,
                            })
                        }
                    };
                }
            } else if let Some(value) = value.cast::<AudioEventCallbackEventTarget>()
                && let Some(value) = unsafe { value.fields() }
                && let Some(value) = value.value.as_ref()
            {
                match value {
                    super::EventTargetInner::ActionType(ty) => {
                        queue::forward(Callback::Filter {
                            id: self.id.get(),
                            target: crate::AnyTarget::Type(*ty),
                            add,
                        });
                    }
                };
            }
        }
        Ref::new_with(|x: &mut Self| {
            x.id.set(self.id.get());
        })
        .unwrap_or_default()
    }

    fn set_lifetime(&self, value: AudioEventCallbackLifetime) -> Ref<Self>
    where
        Self: Sized + ScriptClass,
    {
        queue::forward(Callback::SetLifetime {
            id: self.id.get(),
            sticky: value as i32 > 0,
        });
        Ref::new_with(|x: &mut Self| {
            x.id.set(self.id.get());
        })
        .unwrap_or_default()
    }
}
