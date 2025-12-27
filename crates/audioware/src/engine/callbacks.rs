use std::sync::{
    LazyLock,
    atomic::{AtomicUsize, Ordering},
};

use dashmap::DashMap;
use kira::backend::Backend;
use red4ext_rs::{
    InvokeError, RttiSystem, ScriptClass, ScriptClassOps,
    types::{CName, Function, IScriptable, Ref, TaggedType, WeakRef},
};

use crate::{
    AddContainerStreamingPrefetchEvent, AnyTarget, AudioEventCallbackHandler,
    AudioEventCallbackSystem, ClassName, EngineEmitterEvent, EngineSoundEvent, EngineWwiseEvent,
    EventActionType, EventName, FunctionName, Hydrate, PlayEvent, PlayExternalEvent,
    PlayOneShotEvent, RemoveContainerStreamingPrefetchEvent, SetAppearanceNameEvent,
    SetEntityNameEvent, SetGlobalParameterEvent, SetParameterEvent, SetSwitchEvent, StopSoundEvent,
    StopTaggedEvent, TagEvent, UntagEvent,
    abi::callback::{Callback, FireCallback},
    engine::{Engine, queue},
    utils::{fails, lifecycle, warns},
};

static CALLBACKS: LazyLock<DashMap<Key, AudioEventCallbackWrapper>> =
    LazyLock::new(|| DashMap::with_capacity(128));

static COUNTER: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(0));

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key(usize);

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cb:{}", self.0)
    }
}

pub trait Listen {
    fn register_callback(
        &self,
        event_name: EventName,
        target: WeakRef<IScriptable>,
        function_name: FunctionName,
        id: usize,
    );
    fn register_static_callback(
        &self,
        event_name: EventName,
        class_name: ClassName,
        function_name: FunctionName,
        id: usize,
    );
    fn unregister_callback(&self, id: usize);
    fn filter_callback(&mut self, id: usize, add: bool, target: AnyTarget);
    fn session_reset(&mut self);
    fn set_callback_lifetime(&mut self, id: usize, sticky: bool);
}

pub trait Dispatch {
    fn dispatch(&self, fire: FireCallback);
}

pub trait Call {
    fn call<T: ScriptClassOps>(&self, ctor: impl FnOnce(&mut T) + Copy);
}

impl AudioEventCallbackSystem {
    pub fn any_callback(event_name: EventName, event_type: EventActionType) -> bool {
        CALLBACKS
            .iter()
            .any(|x| x.value().matches(event_name, event_type))
    }
    pub fn is_registered(handler_id: usize) -> bool {
        CALLBACKS.contains_key(&Key(handler_id))
    }
    pub fn dispatch<T: Into<FireCallback>>(event: T) {
        queue::forward(Callback::FireCallbacks(event.into()));
    }
    pub fn register_callback(
        &self,
        event_name: CName,
        target: Ref<IScriptable>,
        function_name: CName,
    ) -> Ref<AudioEventCallbackHandler> {
        let Ok(event_name) = EventName::try_from(event_name) else {
            warns!("event_name is invalid ({})", event_name.as_str());
            return Ref::default();
        };
        let Ok(function_name) = FunctionName::try_from(function_name) else {
            warns!("function_name is invalid ({})", function_name.as_str());
            return Ref::default();
        };
        if target.is_null() {
            warns!("target is null");
            return Ref::default();
        }
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        queue::forward(Callback::RegisterFunction {
            event_name,
            target: target.downgrade().into(),
            function_name,
            id,
        });
        Ref::<AudioEventCallbackHandler>::new_with(|x| {
            x.id.set(id);
        })
        .unwrap_or_default()
    }
    pub fn register_static_callback(
        &self,
        event_name: CName,
        class_name: CName,
        function_name: CName,
    ) -> Ref<AudioEventCallbackHandler> {
        let Ok(event_name) = EventName::try_from(event_name) else {
            warns!("event_name is invalid ({})", event_name.as_str());
            return Ref::default();
        };
        let Ok(class_name) = ClassName::try_from(class_name) else {
            warns!("class_name is invalid ({})", class_name.as_str());
            return Ref::default();
        };
        let Ok(function_name) = FunctionName::try_from(function_name) else {
            warns!("function_name is invalid ({})", function_name.as_str());
            return Ref::default();
        };
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        queue::forward(Callback::RegisterStaticFunction {
            event_name,
            class_name,
            function_name,
            id,
        });
        Ref::<AudioEventCallbackHandler>::new_with(|x| {
            x.id.set(id);
        })
        .unwrap_or_default()
    }
}

impl Dispatch for AudioEventCallback {
    fn dispatch(&self, fire: FireCallback) {
        match self {
            AudioEventCallback::Member(x) => x.dispatch(fire),
            AudioEventCallback::Static(x) => x.dispatch(fire),
        }
    }
}

pub struct AudioEventCallbackWrapper {
    callback: AudioEventCallback,
    targets: Vec<AnyTarget>,
    event_name: EventName,
    sticky: bool,
}

impl Dispatch for AudioEventCallbackWrapper {
    fn dispatch(&self, fire: FireCallback) {
        self.callback.dispatch(fire);
    }
}

pub enum AudioEventCallback {
    Member(MemberFunc),
    Static(StaticFunc),
}

pub struct MemberFunc {
    target: WeakRef<IScriptable>,
    function_name: FunctionName,
}

pub struct StaticFunc {
    class_name: ClassName,
    function_name: FunctionName,
}

impl<T> Dispatch for T
where
    T: Call,
{
    fn dispatch(&self, fire: FireCallback) {
        match &fire {
            FireCallback::Play(f) => self.call(|x: &mut PlayEvent| x.hydrate(f)),
            FireCallback::PlayOneShot(f) => self.call(|x: &mut PlayOneShotEvent| x.hydrate(f)),
            FireCallback::PlayExternal(f) => self.call(|x: &mut PlayExternalEvent| x.hydrate(f)),
            FireCallback::SetGlobalParameter(f) => {
                self.call(|x: &mut SetGlobalParameterEvent| x.hydrate(f))
            }
            FireCallback::SetParameter(f) => self.call(|x: &mut SetParameterEvent| x.hydrate(f)),
            FireCallback::SetSwitch(f) => self.call(|x: &mut SetSwitchEvent| x.hydrate(f)),
            FireCallback::SetAppearanceName(f) => {
                self.call(|x: &mut SetAppearanceNameEvent| x.hydrate(f))
            }
            FireCallback::SetEntityName(f) => self.call(|x: &mut SetEntityNameEvent| x.hydrate(f)),
            FireCallback::Stop(f) => self.call(|x: &mut StopSoundEvent| x.hydrate(f)),
            FireCallback::StopTagged(f) => self.call(|x: &mut StopTaggedEvent| x.hydrate(f)),
            FireCallback::Tag(f) => self.call(|x: &mut TagEvent| x.hydrate(f)),
            FireCallback::Untag(f) => self.call(|x: &mut UntagEvent| x.hydrate(f)),
            FireCallback::AddContainerStreamingPrefetch(f) => {
                self.call(|x: &mut AddContainerStreamingPrefetchEvent| x.hydrate(f))
            }
            FireCallback::RemoveContainerStreamingPrefetch(f) => {
                self.call(|x: &mut RemoveContainerStreamingPrefetchEvent| x.hydrate(f))
            }
        }
    }
}

impl Call for MemberFunc {
    fn call<T: ScriptClassOps>(&self, ctor: impl FnOnce(&mut T) + Copy) {
        let rtti = RttiSystem::get();
        if let Some(target) = self.target.clone().upgrade()
            && let Some(class_name) = unsafe { target.instance() }.map(|x| x.class().name())
        {
            if let Some(class) = rtti.get_class(class_name) {
                if let Some(func) = class.methods().iter().map(|x| x.as_function()).find(|x| {
                    x.name() == *self.function_name && x.flags().is_event() && x.params().len() == 1
                }) {
                    inner_call::<T>(unsafe { target.fields() }, func, ctor);
                }
            } else {
                warns!("could not find callback class");
            }
        }
    }
}

impl Call for StaticFunc {
    fn call<T: ScriptClassOps>(&self, ctor: impl FnOnce(&mut T) + Copy) {
        let rtti = RttiSystem::get();
        let full_name = format!(
            "{}::{}",
            (*self.class_name).as_str(),
            (*self.function_name).as_str()
        );
        if let Some(func) = rtti.get_global_functions().iter().find(|x| {
            x.name().as_str() == full_name && x.flags().is_event() && x.params().len() == 1
        }) {
            inner_call::<T>(None, func, ctor);
        } else {
            warns!("could not find static callback {full_name}");
        }
    }
}

fn inner_call<T: ScriptClass>(
    receiver: Option<&IScriptable>,
    func: &Function,
    ctor: impl FnOnce(&mut T) + Copy,
) {
    let Some(event) = T::new_ref_with(|x| {
        ctor(x);
    }) else {
        return;
    };
    let Some(param) = func.params().first() else {
        fails!("callback expects a single event param");
        return;
    };
    let TaggedType::Ref(param_tag) = param.type_().tagged() else {
        fails!("callback event param should be a ref");
        return;
    };
    let Some(param_class) = param_tag.pointee().as_class() else {
        fails!("callback event param ref should point to a class");
        return;
    };
    if let Err(e) = match param_class.name().as_str() {
        x if x == T::NAME => func.execute::<_, ()>(receiver, (event,)),
        EngineSoundEvent::NAME => event
            .cast::<EngineSoundEvent>()
            .map(|x| func.execute::<_, ()>(receiver, (x,)))
            .unwrap_or(Err(InvokeError::ArgMismatch {
                function: func.name().as_str(),
                expected: EngineSoundEvent::NAME,
                index: 0,
            })),
        EngineWwiseEvent::NAME => event
            .cast::<EngineWwiseEvent>()
            .map(|x| func.execute::<_, ()>(receiver, (x,)))
            .unwrap_or(Err(InvokeError::ArgMismatch {
                function: func.name().as_str(),
                expected: EngineWwiseEvent::NAME,
                index: 0,
            })),
        EngineEmitterEvent::NAME => event
            .cast::<EngineEmitterEvent>()
            .map(|x| func.execute::<_, ()>(receiver, (x,)))
            .unwrap_or(Err(InvokeError::ArgMismatch {
                function: func.name().as_str(),
                expected: EngineEmitterEvent::NAME,
                index: 0,
            })),
        _ => Err(InvokeError::ArgMismatch {
            function: func.name().as_str(),
            expected: T::NAME,
            index: 0,
        }),
    } {
        fails!("couldn't execute callback: {e}");
    }
}

impl<B: Backend> Listen for Engine<B> {
    fn register_callback(
        &self,
        event_name: EventName,
        target: WeakRef<IScriptable>,
        function_name: FunctionName,
        id: usize,
    ) {
        let callback = AudioEventCallback::Member(MemberFunc {
            target,
            function_name,
        });
        let value = AudioEventCallbackWrapper {
            callback,
            targets: Vec::with_capacity(5),
            event_name,
            sticky: false,
        };
        let id = Key(id);
        if CALLBACKS.get(&id).is_some() {
            warns!("{id} callback already registered for {event_name} => {function_name}");
        } else {
            let _ = CALLBACKS.insert(id, value);
            lifecycle!("{id} registered new callback for {event_name} => {function_name}",);
        }
    }

    fn register_static_callback(
        &self,
        event_name: EventName,
        class_name: ClassName,
        function_name: FunctionName,
        id: usize,
    ) {
        let callback = AudioEventCallback::Static(StaticFunc {
            class_name,
            function_name,
        });
        let value = AudioEventCallbackWrapper {
            callback,
            targets: Vec::with_capacity(5),
            event_name,
            sticky: false,
        };
        let id = Key(id);
        if CALLBACKS.get(&id).is_some() {
            warns!(
                "{id} static callback already registered for {event_name} => {class_name} {function_name}"
            );
        } else {
            let _ = CALLBACKS.insert(id, value);
            lifecycle!(
                "{id} registered new static callback for {event_name} => {class_name} {function_name}",
            );
        }
    }

    fn unregister_callback(&self, id: usize) {
        CALLBACKS.remove(&Key(id));
    }

    fn filter_callback(&mut self, id: usize, add: bool, target: AnyTarget) {
        if let Some(mut cb) = CALLBACKS.get_mut(&Key(id)) {
            if add && !cb.targets.contains(&target) {
                cb.targets.push(target);
            } else if !add && let Some(idx) = cb.targets.iter().position(|x| *x == target) {
                cb.targets.remove(idx);
            }
        }
    }

    fn session_reset(&mut self) {
        CALLBACKS.retain(|_, v| v.sticky);
    }

    fn set_callback_lifetime(&mut self, id: usize, sticky: bool) {
        if let Some(mut cb) = CALLBACKS.get_mut(&Key(id)) {
            cb.sticky = sticky;
        }
    }
}

impl<B: Backend> Dispatch for Engine<B> {
    fn dispatch(&self, fire: FireCallback) {
        for cb in CALLBACKS
            .iter()
            .filter(|x| x.value().matches_filters(&fire))
        {
            cb.dispatch(fire.clone());
        }
    }
}

impl AudioEventCallbackWrapper {
    pub fn matches(&self, event_name: EventName, event_type: EventActionType) -> bool {
        self.event_name == event_name
            && (self
                .targets
                .iter()
                .filter(|t| matches!(t, AnyTarget::Type(_)))
                .count()
                == 0
                || self.targets.iter().any(|t| match t {
                    AnyTarget::Type(x) => *x == event_type,
                    _ => false,
                }))
    }
    pub fn matches_filters(&self, other: &FireCallback) -> bool {
        self.matches(other.event_name(), other.event_type())
            && (self
                .targets
                .iter()
                .filter(|t| matches!(t, AnyTarget::Id(_)))
                .count()
                == 0
                || self.targets.iter().any(|t| match t {
                    AnyTarget::Id(x) => other.entity_id().map(|y| *x == y).unwrap_or(true),
                    _ => false,
                }))
            && (self
                .targets
                .iter()
                .filter(|t| matches!(t, AnyTarget::EmitterName(_)))
                .count()
                == 0
                || self.targets.iter().any(|t| match t {
                    AnyTarget::EmitterName(x) => {
                        other.emitter_name().map(|y| *x == y).unwrap_or(true)
                    }
                    _ => false,
                }))
    }
}
