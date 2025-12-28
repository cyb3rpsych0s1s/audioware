use debug_ignore::DebugIgnore;
use red4ext_rs::types::{CName, EntityId, IScriptable, ResRef, WeakRef};

use crate::{
    AnyTarget, AudioEventCallbackLifetime, ClassName, ESoundCurveType, EventActionType, EventName,
    FunctionName, Pair, TFlag, Vector4, WwiseId,
};

#[derive(Debug)]
pub enum Callback {
    RegisterFunction {
        event_name: EventName,
        target: DebugIgnore<WeakRef<IScriptable>>,
        function_name: FunctionName,
        id: usize,
    },
    RegisterStaticFunction {
        event_name: EventName,
        class_name: ClassName,
        function_name: FunctionName,
        id: usize,
    },
    FireCallbacks(FireCallback),
    Unregister {
        id: usize,
    },
    Filter {
        id: usize,
        target: AnyTarget,
        add: bool,
    },
    SetLifetime {
        id: usize,
        sticky: bool,
    },
}

#[derive(Debug, Clone)]
pub enum FireCallback {
    Play(FirePlayCallback),
    PlayExternal(FirePlayExternalCallback),
    PlayOneShot(FirePlayOneShotCallback),
    SetGlobalParameter(FireSetGlobalParameterCallback),
    SetParameter(FireSetParameterCallback),
    SetSwitch(FireSetSwitchCallback),
    SetAppearanceName(FireSetAppearanceNameCallback),
    SetEntityName(FireSetEntityNameCallback),
    Stop(FireStopCallback),
    StopTagged(FireStopTaggedCallback),
    Tag(FireTagCallback),
    Untag(FireUntagCallback),
    AddContainerStreamingPrefetch(FireAddContainerStreamingPrefetchCallback),
    RemoveContainerStreamingPrefetch(FireRemoveContainerStreamingPrefetchCallback),
}

impl FireCallback {
    pub fn event_name(&self) -> EventName {
        match self {
            Self::Play(FirePlayCallback { event_name, .. })
            | Self::PlayOneShot(FirePlayOneShotCallback {
                base: FirePlayCallback { event_name, .. },
                ..
            })
            | Self::PlayExternal(FirePlayExternalCallback {
                base: FirePlayCallback { event_name, .. },
                ..
            })
            | Self::SetGlobalParameter(FireSetGlobalParameterCallback {
                name: event_name, ..
            })
            | Self::SetParameter(FireSetParameterCallback {
                base: FirePlayCallback { event_name, .. },
                ..
            })
            | Self::SetSwitch(FireSetSwitchCallback {
                base: FirePlayCallback { event_name, .. },
                ..
            })
            | Self::SetAppearanceName(FireSetAppearanceNameCallback { event_name, .. })
            | Self::SetEntityName(FireSetEntityNameCallback { event_name, .. })
            | Self::Stop(FireStopCallback { event_name, .. })
            | Self::StopTagged(FireStopTaggedCallback { event_name, .. })
            | Self::Tag(FireTagCallback { event_name, .. })
            | Self::Untag(FireUntagCallback { event_name, .. })
            | Self::AddContainerStreamingPrefetch(FireAddContainerStreamingPrefetchCallback {
                event_name,
                ..
            })
            | Self::RemoveContainerStreamingPrefetch(
                FireRemoveContainerStreamingPrefetchCallback { event_name, .. },
            ) => *event_name,
        }
    }
    pub fn event_type(&self) -> EventActionType {
        match self {
            Self::Play(FirePlayCallback { event_type, .. })
            | Self::PlayOneShot(FirePlayOneShotCallback {
                base: FirePlayCallback { event_type, .. },
                ..
            }) => *event_type,
            Self::PlayExternal(FirePlayExternalCallback { .. }) => EventActionType::PlayExternal,
            Self::SetGlobalParameter(_) | Self::SetParameter(_) => EventActionType::SetParameter,
            Self::SetSwitch(_) => EventActionType::SetSwitch,
            Self::SetAppearanceName(_) => EventActionType::SetAppearanceName,
            Self::SetEntityName(_) => EventActionType::SetEntityName,
            Self::Stop(_) => EventActionType::StopSound,
            Self::StopTagged(_) => EventActionType::StopTagged,
            Self::Tag(_) => EventActionType::Tag,
            Self::Untag(_) => EventActionType::Untag,
            Self::AddContainerStreamingPrefetch(_) => {
                EventActionType::AddContainerStreamingPrefetch
            }
            Self::RemoveContainerStreamingPrefetch(_) => {
                EventActionType::RemoveContainerStreamingPrefetch
            }
        }
    }
    pub fn entity_id(&self) -> Option<EntityId> {
        match self {
            Self::Play(x) => Some(x.entity_id),
            Self::PlayExternal(x) => Some(x.base.entity_id),
            Self::PlayOneShot(x) => Some(x.base.entity_id),
            Self::SetParameter(x) => Some(x.base.entity_id),
            Self::SetSwitch(x) => Some(x.base.entity_id),
            Self::SetAppearanceName(x) => Some(x.entity_id),
            Self::SetEntityName(x) => Some(x.entity_id),
            Self::Stop(x) => Some(x.entity_id),
            Self::StopTagged(x) => Some(x.entity_id),
            Self::Tag(x) => Some(x.entity_id),
            Self::Untag(x) => Some(x.entity_id),
            Self::AddContainerStreamingPrefetch(x) => Some(x.entity_id),
            Self::RemoveContainerStreamingPrefetch(x) => Some(x.entity_id),
            Self::SetGlobalParameter(_) => None,
        }
    }
    pub fn emitter_name(&self) -> Option<CName> {
        match self {
            Self::Play(x) => Some(x.emitter_name),
            Self::PlayExternal(x) => Some(x.base.emitter_name),
            Self::PlayOneShot(x) => Some(x.base.emitter_name),
            Self::SetParameter(x) => Some(x.base.emitter_name),
            Self::SetSwitch(x) => Some(x.base.emitter_name),
            Self::SetAppearanceName(_)
            | Self::SetEntityName(_)
            | Self::Stop(_)
            | Self::StopTagged(_)
            | Self::Tag(_)
            | Self::Untag(_)
            | Self::AddContainerStreamingPrefetch(_)
            | Self::RemoveContainerStreamingPrefetch(_)
            | Self::SetGlobalParameter(_) => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FireSetAppearanceNameCallback {
    pub event_name: EventName,
    pub entity_id: EntityId,
    pub wwise_id: WwiseId,
}

#[derive(Debug, Clone)]
pub struct FireSetEntityNameCallback {
    pub event_name: EventName,
    pub entity_id: EntityId,
    pub wwise_id: WwiseId,
}

#[derive(Debug, Clone)]
pub struct FirePlayCallback {
    pub event_name: EventName,
    pub event_type: EventActionType,
    pub entity_id: EntityId,
    pub emitter_name: CName,
    pub sound_tags: Vec<CName>,
    pub emitter_tags: Vec<CName>,
    pub wwise_id: WwiseId,
    pub seek: f32,
    pub position: Vector4,
    pub has_position: bool,
}

#[derive(Debug, Clone)]
pub struct FirePlayExternalCallback {
    pub base: FirePlayCallback,
    pub external_resource_path: ResRef,
}

impl AsRef<FirePlayCallback> for FirePlayExternalCallback {
    fn as_ref(&self) -> &FirePlayCallback {
        &self.base
    }
}

#[derive(Debug, Clone)]
pub struct FirePlayOneShotCallback {
    pub base: FirePlayCallback,
    pub params: Vec<Pair<f32>>,
    pub switches: Vec<Pair<CName>>,
    pub graph_occlusion: f32,
    pub raycast_occlusion: f32,
    pub flags: TFlag,
}

impl FirePlayOneShotCallback {
    pub fn params(&self) -> &[Pair<f32>] {
        self.params.as_slice()
    }
    pub fn switches(&self) -> &[Pair<CName>] {
        self.switches.as_slice()
    }
}

impl AsRef<FirePlayCallback> for FirePlayOneShotCallback {
    fn as_ref(&self) -> &FirePlayCallback {
        &self.base
    }
}

#[derive(Debug, Clone)]
pub struct FireSetGlobalParameterCallback {
    pub name: EventName,
    pub value: f32,
    pub duration: f32,
    pub curve_type: ESoundCurveType,
    pub wwise_id: WwiseId,
}

#[derive(Debug, Clone)]
pub struct FireSetParameterCallback {
    pub base: FirePlayCallback,
    pub switch_name: CName,
    pub switch_value: f32,
}

impl AsRef<FirePlayCallback> for FireSetParameterCallback {
    fn as_ref(&self) -> &FirePlayCallback {
        &self.base
    }
}

#[derive(Debug, Clone)]
pub struct FireSetSwitchCallback {
    pub base: FirePlayCallback,
    pub switch_name: CName,
    pub switch_value: CName,
    pub switch_name_wwise_id: WwiseId,
    pub switch_value_wwise_id: WwiseId,
}

impl AsRef<FirePlayCallback> for FireSetSwitchCallback {
    fn as_ref(&self) -> &FirePlayCallback {
        &self.base
    }
}

#[derive(Debug, Clone)]
pub struct FireStopCallback {
    pub event_name: EventName,
    pub event_type: EventActionType,
    pub entity_id: EntityId,
    pub float_data: f32,
    pub wwise_id: WwiseId,
}

#[derive(Debug, Clone)]
pub struct FireStopTaggedCallback {
    pub event_name: EventName,
    pub event_type: EventActionType,
    pub entity_id: EntityId,
    pub float_data: f32,
    pub wwise_id: WwiseId,
}

#[derive(Debug, Clone)]
pub struct FireTagCallback {
    pub event_name: EventName,
    pub event_type: EventActionType,
    pub entity_id: EntityId,
    pub wwise_id: WwiseId,
}

#[derive(Debug, Clone)]
pub struct FireUntagCallback {
    pub event_name: EventName,
    pub event_type: EventActionType,
    pub entity_id: EntityId,
    pub wwise_id: WwiseId,
}

#[derive(Debug, Clone)]
pub struct FireAddContainerStreamingPrefetchCallback {
    pub event_name: EventName,
    pub event_type: EventActionType,
    pub entity_id: EntityId,
    pub wwise_id: WwiseId,
}

#[derive(Debug, Clone)]
pub struct FireRemoveContainerStreamingPrefetchCallback {
    pub event_name: EventName,
    pub event_type: EventActionType,
    pub entity_id: EntityId,
    pub wwise_id: WwiseId,
}

impl std::fmt::Display for Callback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RegisterFunction {
                event_name,
                function_name,
                ..
            } => write!(
                f,
                "register callback for {event_name} to instance class method {function_name}",
            ),
            Self::RegisterStaticFunction {
                event_name,
                class_name,
                function_name,
                ..
            } => write!(
                f,
                "register callback for {event_name} to class {class_name} static method {function_name}",
            ),
            Self::Filter { add, .. } => {
                write!(f, "{} filter", if *add { "add" } else { "remove" })
            }
            Self::SetLifetime { sticky, .. } => {
                write!(
                    f,
                    "set lifetime {}",
                    if *sticky {
                        AudioEventCallbackLifetime::Forever
                    } else {
                        AudioEventCallbackLifetime::Session
                    }
                )
            }
            Self::FireCallbacks(FireCallback::Play(FirePlayCallback {
                event_name,
                event_type,
                ..
            }))
            | Self::FireCallbacks(FireCallback::PlayExternal(FirePlayExternalCallback {
                base:
                    FirePlayCallback {
                        event_name,
                        event_type,
                        ..
                    },
                ..
            }))
            | Self::FireCallbacks(FireCallback::PlayOneShot(FirePlayOneShotCallback {
                base:
                    FirePlayCallback {
                        event_name,
                        event_type,
                        ..
                    },
                ..
            }))
            | Self::FireCallbacks(FireCallback::SetParameter(FireSetParameterCallback {
                base:
                    FirePlayCallback {
                        event_name,
                        event_type,
                        ..
                    },
                ..
            }))
            | Self::FireCallbacks(FireCallback::SetSwitch(FireSetSwitchCallback {
                base:
                    FirePlayCallback {
                        event_name,
                        event_type,
                        ..
                    },
                ..
            }))
            | Self::FireCallbacks(FireCallback::Stop(FireStopCallback {
                event_name,
                event_type,
                ..
            }))
            | Self::FireCallbacks(FireCallback::StopTagged(FireStopTaggedCallback {
                event_name,
                event_type,
                ..
            })) => {
                write!(
                    f,
                    "fire callback(s) for {event_type} event for {event_name}",
                )
            }
            Self::FireCallbacks(FireCallback::SetGlobalParameter(
                FireSetGlobalParameterCallback { name, .. },
            )) => write!(f, "fire callback(s) for global parameter event for {name}",),
            Self::FireCallbacks(FireCallback::SetAppearanceName(
                FireSetAppearanceNameCallback { event_name, .. },
            )) => write!(
                f,
                "fire callback(s) for set appearance name event for {event_name}",
            ),
            Self::FireCallbacks(FireCallback::SetEntityName(FireSetEntityNameCallback {
                event_name,
                ..
            })) => write!(
                f,
                "fire callback(s) for set entity name event for {event_name}",
            ),
            Self::FireCallbacks(FireCallback::Tag(FireTagCallback { event_name, .. })) => {
                write!(f, "fire callback(s) for tag event for {event_name}",)
            }
            Self::FireCallbacks(FireCallback::Untag(FireUntagCallback { event_name, .. })) => {
                write!(f, "fire callback(s) for untag event for {event_name}",)
            }
            Self::FireCallbacks(FireCallback::AddContainerStreamingPrefetch(
                FireAddContainerStreamingPrefetchCallback { event_name, .. },
            )) => write!(
                f,
                "fire callback(s) for add container streaming prefetch event for {event_name}",
            ),
            Self::FireCallbacks(FireCallback::RemoveContainerStreamingPrefetch(
                FireRemoveContainerStreamingPrefetchCallback { event_name, .. },
            )) => write!(
                f,
                "fire callback(s) for remove container streaming prefetch event for {event_name}",
            ),
            Self::Unregister { .. } => write!(f, "unregister callback"),
        }
    }
}
