use red4ext_rs::types::CName;
use red4ext_rs::{SdkEnv, VoidPtr};

pub fn attach_hooks(env: &SdkEnv) {
    event::attach_hook(env);
    post_event::attach_hook(env);
    post_event::oneshot::attach_hook(env);
    external_event::attach_hook(env);
    parameter::global::attach_hook(env);
    parameter::attach_hook(env);
    set_switch::attach_hook(env);
}

pub mod post_event {
    use crate::{
        AudioEventCallbackSystem, PlayingSoundId, Sound, SoundEngine,
        abi::callback::{FireCallback, FirePlayCallback, FirePlayExternalCallback},
        engine::{Mute, Replacements},
    };

    use super::*;

    ::red4ext_rs::hooks! {
        static HOOK: fn(a1: VoidPtr,
        a2: CName,
        a3: *mut Sound) -> PlayingSoundId;
    }

    #[allow(clippy::missing_transmute_annotations)]
    pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
        let addr =
            ::red4ext_rs::addr_hashes::resolve(super::super::offsets::SOUNDENGINE_POST_EVENT);
        let addr = unsafe { ::std::mem::transmute(addr) };
        unsafe { env.attach_hook(HOOK, addr, detour) };
        crate::utils::intercept!(
            "attached native internal hook for SoundEngine::PostEvent( CName, VoidPtr )"
        );
    }

    unsafe extern "C" fn detour(
        a1: VoidPtr,
        a2: CName,
        a3: *mut Sound,
        cb: unsafe extern "C" fn(a1: VoidPtr, a2: CName, a3: *mut Sound) -> PlayingSoundId,
    ) -> PlayingSoundId {
        unsafe {
            let sound = &*a3;
            let Ok(event_name) = sound.sound_name().try_into() else {
                return cb(a1, a2, a3);
            };
            let wwise_id = SoundEngine::get()
                .metadata_manager()
                .wwise_id(sound.sound_name());
            let event_type = if sound.is_play_external() {
                crate::EventActionType::PlayExternal
            } else {
                crate::EventActionType::Play
            };

            if AudioEventCallbackSystem::any_callback(event_name, crate::EventActionType::Play)
                || AudioEventCallbackSystem::any_callback(
                    event_name,
                    crate::EventActionType::PlayAnimation,
                )
                || AudioEventCallbackSystem::any_callback(
                    event_name,
                    crate::EventActionType::PlayExternal,
                )
            {
                let sound_object = sound.sound_object();
                let entity_id = sound_object.map(|x| x.entity_id()).unwrap_or_default();
                let emitter_name = sound_object.map(|x| x.emitter_name()).unwrap_or_default();
                let sound_tags = sound_object
                    .map(|x| x.sound_tags().map(|x| x.to_vec()).unwrap_or_default())
                    .unwrap_or_default();
                let emitter_tags = sound_object
                    .map(|x| x.emitter_tags().map(|x| x.to_vec()).unwrap_or_default())
                    .unwrap_or_default();
                if sound.is_play_external() {
                    AudioEventCallbackSystem::dispatch(FireCallback::PlayExternal(
                        FirePlayExternalCallback {
                            base: FirePlayCallback {
                                event_name,
                                event_type: crate::EventActionType::PlayExternal,
                                entity_id,
                                emitter_name,
                                sound_tags,
                                emitter_tags,
                                wwise_id,
                            },
                            external_resource_path: sound.external_resource_path(),
                        },
                    ));
                } else {
                    AudioEventCallbackSystem::dispatch(FireCallback::Play(FirePlayCallback {
                        event_name,
                        event_type: crate::EventActionType::Play,
                        entity_id,
                        emitter_name,
                        sound_tags,
                        emitter_tags,
                        wwise_id,
                    }));
                }
            }
            if !Replacements.is_specific_muted(event_name, event_type) {
                crate::utils::intercept!("SoundEngine::PostEvent( {a2}, {sound} ) / {wwise_id}",);
                return cb(a1, a2, a3);
            }
            PlayingSoundId::invalid()
        }
    }

    pub mod oneshot {

        use std::ops::Not;

        use crate::{OneShotSound, SoundObject, abi::callback::FirePlayOneShotCallback};

        use super::*;

        ::red4ext_rs::hooks! {
            static HOOK: fn(a1: VoidPtr,
            a2: *mut OneShotSound,
            a3: *mut SoundObject) -> PlayingSoundId;
        }

        #[allow(clippy::missing_transmute_annotations)]
        pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
            let addr = ::red4ext_rs::addr_hashes::resolve(
                super::super::super::offsets::SOUNDENGINE_POST_EVENT_ONESHOT,
            );
            let addr = unsafe { ::std::mem::transmute(addr) };
            unsafe { env.attach_hook(HOOK, addr, detour) };
            crate::utils::intercept!(
                "attached native internal hook for SoundEngine::PostEvent_OneShot( VoidPtr, VoidPtr )"
            );
        }

        unsafe extern "C" fn detour(
            a1: VoidPtr,
            a2: *mut OneShotSound,
            a3: *mut SoundObject,
            cb: unsafe extern "C" fn(
                a1: VoidPtr,
                a2: *mut OneShotSound,
                a3: *mut SoundObject,
            ) -> PlayingSoundId,
        ) -> PlayingSoundId {
            unsafe {
                if !a2.is_null() && !a2.byte_offset(48).is_null() {
                    let event_type = crate::EventActionType::Play;
                    let oneshot = &*a2;
                    let wwise_id = oneshot.wwise_id();
                    let Ok(event_name) = oneshot.event_name().try_into() else {
                        return cb(a1, a2, a3);
                    };
                    let sound_object = a3.is_null().not().then(|| &*a3);
                    let entity_id = sound_object.map(|x| x.entity_id()).unwrap_or_default();
                    let emitter_name = sound_object.map(|x| x.emitter_name()).unwrap_or_default();
                    let sound_tags = sound_object
                        .map(|x| x.sound_tags().map(|x| x.to_vec()).unwrap_or_default())
                        .unwrap_or_default();
                    let emitter_tags = sound_object
                        .map(|x| x.emitter_tags().map(|x| x.to_vec()).unwrap_or_default())
                        .unwrap_or_default();
                    let params = oneshot.params().to_vec();
                    let switches = oneshot.switches().to_vec();

                    if AudioEventCallbackSystem::any_callback(event_name, event_type) {
                        AudioEventCallbackSystem::dispatch(FireCallback::PlayOneShot(
                            FirePlayOneShotCallback {
                                base: FirePlayCallback {
                                    event_name,
                                    event_type,
                                    entity_id,
                                    emitter_name,
                                    sound_tags,
                                    emitter_tags,
                                    wwise_id,
                                },
                                params,
                                switches,
                            },
                        ));
                    }
                    if Replacements.is_specific_muted(event_name, event_type) {
                        return PlayingSoundId::invalid();
                    }
                    crate::utils::intercept!(
                        "SoundEngine::PostEvent_OneShot( {{ {} }}, {{ {} }} ) / {wwise_id}",
                        oneshot,
                        sound_object
                            .map(|x| format!("{x}"))
                            .unwrap_or("..".to_string())
                    );
                }
                cb(a1, a2, a3)
            }
        }
    }
}

pub mod external_event {
    use std::ops::Not;

    use red4ext_rs::types::ResRef;

    use crate::{
        AudioEventCallbackSystem, EventName, PlayingSoundId, Sound, SoundEngine,
        abi::callback::{FireCallback, FirePlayCallback, FirePlayExternalCallback},
        engine::{Mute, Replacements},
    };

    use super::*;

    ::red4ext_rs::hooks! {
        static HOOK: fn(a1: VoidPtr,
        a2: CName,
        a3: *const ResRef,
        a4: *mut Sound) -> PlayingSoundId;
    }

    #[allow(clippy::missing_transmute_annotations)]
    pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
        let addr = ::red4ext_rs::addr_hashes::resolve(
            super::super::offsets::SOUNDENGINE_EXTERNAL_EVENT_RES,
        );
        let addr = unsafe { ::std::mem::transmute(addr) };
        unsafe { env.attach_hook(HOOK, addr, detour) };
        crate::utils::intercept!(
            "attached native internal hook for SoundEngine::ExternalEvent( CName, VoidPtr )"
        );
    }

    unsafe extern "C" fn detour(
        a1: VoidPtr,
        a2: CName,
        a3: *const ResRef,
        a4: *mut Sound,
        cb: unsafe extern "C" fn(
            a1: VoidPtr,
            a2: CName,
            a3: *const ResRef,
            a4: *mut Sound,
        ) -> PlayingSoundId,
    ) -> PlayingSoundId {
        unsafe {
            if a4.is_null().not() && a3.is_null().not() {
                let Ok(event_name) = EventName::try_from(a2) else {
                    return cb(a1, a2, a3, a4);
                };
                let sound = &*a4;
                let event_type = crate::EventActionType::SetParameter;
                let engine = SoundEngine::get();
                let sound_object = sound.sound_object();
                let wwise_id = engine.metadata_manager().game_parameter_id(a2);
                let entity_id = sound_object.map(|x| x.entity_id()).unwrap_or_default();
                let emitter_name = sound_object.map(|x| x.emitter_name()).unwrap_or_default();
                let sound_tags = sound_object
                    .map(|x| x.sound_tags().map(|x| x.to_vec()).unwrap_or_default())
                    .unwrap_or_default();
                let emitter_tags = sound_object
                    .map(|x| x.emitter_tags().map(|x| x.to_vec()).unwrap_or_default())
                    .unwrap_or_default();
                if AudioEventCallbackSystem::any_callback(event_name, event_type) {
                    AudioEventCallbackSystem::dispatch(FireCallback::PlayExternal(
                        FirePlayExternalCallback {
                            base: FirePlayCallback {
                                event_name,
                                event_type: crate::EventActionType::PlayExternal,
                                entity_id,
                                emitter_name,
                                wwise_id,
                                sound_tags,
                                emitter_tags,
                            },
                            external_resource_path: if a3.is_null().not() {
                                (*a3).clone()
                            } else {
                                sound.external_resource_path()
                            },
                        },
                    ));
                }
                if Replacements.is_specific_muted(event_name, event_type) {
                    return PlayingSoundId::invalid();
                }
                crate::utils::intercept!(
                    "SoundEngine::ExternalEvent( {{ {a2} }}, .., {{ {} }} ) / {wwise_id}",
                    &*a4,
                );
            }
            cb(a1, a2, a3, a4)
        }
    }
}

pub mod parameter {
    use crate::{
        AudioEventCallbackSystem, EventName, SoundEngine, SoundObjectId,
        abi::callback::{FireCallback, FirePlayCallback, FireSetParameterCallback},
        engine::{Mute, Replacements},
    };

    use super::*;

    pub mod global {
        use super::*;
        use crate::{
            ESoundCurveType, WwiseId,
            abi::callback::{FireCallback, FireSetGlobalParameterCallback},
        };

        ::red4ext_rs::hooks! {
            static HOOK: fn(a1: CName,
            a2: f32,
            a3: f32,
            a4: ESoundCurveType) -> i64;
        }

        #[allow(clippy::missing_transmute_annotations)]
        pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
            let addr = ::red4ext_rs::addr_hashes::resolve(
                super::super::super::offsets::SOUNDENGINE_SET_GLOBAL_PARAMETER,
            );
            let addr = unsafe { ::std::mem::transmute(addr) };
            unsafe { env.attach_hook(HOOK, addr, detour) };
            crate::utils::intercept!(
                "attached native internal hook for SoundEngine::SetGlobalParameter( CName, CName, Float, CurveType )"
            );
        }

        unsafe extern "C" fn detour(
            a1: CName,
            a2: f32,
            a3: f32,
            a4: ESoundCurveType,
            cb: unsafe extern "C" fn(a1: CName, a2: f32, a3: f32, a4: ESoundCurveType) -> i64,
        ) -> i64 {
            unsafe {
                let wwise_id = SoundEngine::get().metadata_manager().game_parameter_id(a1);
                if !wwise_id.is_null() {
                    let Ok(event_name) = EventName::try_from(a1) else {
                        return cb(a1, a2, a3, a4);
                    };
                    let event_type = crate::EventActionType::SetParameter;
                    if AudioEventCallbackSystem::any_callback(event_name, event_type) {
                        AudioEventCallbackSystem::dispatch(FireCallback::SetGlobalParameter(
                            FireSetGlobalParameterCallback {
                                name: event_name,
                                value: a2,
                                duration: a3,
                                curve_type: a4,
                                wwise_id,
                            },
                        ));
                    }
                    if Replacements.is_specific_muted(event_name, event_type) {
                        return WwiseId::default().to_i64();
                    }
                    // crate::utils::intercept!(
                    //     "SoundEngine::SetGlobalParameter( {a1}, {a2}, {a3}, {a4} ) / {wwise_id}"
                    // );
                }
                cb(a1, a2, a3, a4)
            }
        }
    }

    ::red4ext_rs::hooks! {
        static HOOK: fn(a1: VoidPtr,
        a2: CName,
        a3: f32,
        a4: SoundObjectId) -> i64;
    }

    #[allow(clippy::missing_transmute_annotations)]
    pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
        let addr =
            ::red4ext_rs::addr_hashes::resolve(super::super::offsets::SOUNDENGINE_SET_PARAMETER);
        let addr = unsafe { ::std::mem::transmute(addr) };
        unsafe { env.attach_hook(HOOK, addr, detour) };
        crate::utils::intercept!(
            "attached native internal hook for SoundEngine::SetParameter( CName, Float, SoundObjectId )"
        );
    }

    unsafe extern "C" fn detour(
        a1: VoidPtr,
        a2: CName,
        a3: f32,
        a4: SoundObjectId,
        cb: unsafe extern "C" fn(a1: VoidPtr, a2: CName, a3: f32, a4: SoundObjectId) -> i64,
    ) -> i64 {
        unsafe {
            let Ok(event_name) = EventName::try_from(a2) else {
                return cb(a1, a2, a3, a4);
            };
            let event_type = crate::EventActionType::SetParameter;
            let engine = SoundEngine::get();
            let manager = engine.sound_object_manager();
            let sound_object = manager.sound_object(a4);
            let wwise_id = engine.metadata_manager().game_parameter_id(a2);
            let entity_id = sound_object.map(|x| x.entity_id()).unwrap_or_default();
            let emitter_name = sound_object.map(|x| x.emitter_name()).unwrap_or_default();
            let sound_tags = sound_object
                .map(|x| x.sound_tags().map(|x| x.to_vec()).unwrap_or_default())
                .unwrap_or_default();
            let emitter_tags = sound_object
                .map(|x| x.emitter_tags().map(|x| x.to_vec()).unwrap_or_default())
                .unwrap_or_default();

            if AudioEventCallbackSystem::any_callback(event_name, event_type) {
                AudioEventCallbackSystem::dispatch(FireCallback::SetParameter(
                    FireSetParameterCallback {
                        base: FirePlayCallback {
                            event_name,
                            event_type,
                            emitter_name,
                            entity_id,
                            sound_tags,
                            emitter_tags,
                            wwise_id,
                        },
                        switch_name: a2,
                        switch_value: a3,
                    },
                ));
            }
            if !Replacements.is_specific_muted(event_name, event_type) {
                // crate::utils::intercept!(
                //     "SoundEngine::SetParameter( {a2}, {a3}, {a4} ) / {wwise_id}"
                // );
                return cb(a1, a2, a3, a4);
            }
            0
        }
    }
}

pub mod set_switch {
    use crate::{
        AudioEventCallbackSystem, EventName, SoundEngine, SoundObjectId,
        abi::callback::{FireCallback, FirePlayCallback, FireSetSwitchCallback},
        engine::{Mute, Replacements},
    };

    use super::*;

    ::red4ext_rs::hooks! {
        static HOOK: fn(a1: VoidPtr,
        a2: CName,
        a3: CName,
        a4: SoundObjectId) -> i64;
    }

    #[allow(clippy::missing_transmute_annotations)]
    pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
        let addr =
            ::red4ext_rs::addr_hashes::resolve(super::super::offsets::SOUNDENGINE_SET_SWITCH);
        let addr = unsafe { ::std::mem::transmute(addr) };
        unsafe { env.attach_hook(HOOK, addr, detour) };
        crate::utils::intercept!(
            "attached native internal hook for SoundEngine::SetSwitch( CName, CName, SoundObjectId )"
        );
    }

    unsafe extern "C" fn detour(
        a1: VoidPtr,
        a2: CName,
        a3: CName,
        a4: SoundObjectId,
        cb: unsafe extern "C" fn(a1: VoidPtr, a2: CName, a3: CName, a4: SoundObjectId) -> i64,
    ) -> i64 {
        unsafe {
            let event_type = crate::EventActionType::SetSwitch;
            let engine = SoundEngine::get();
            let Ok(event_name) = EventName::try_from(a2) else {
                return cb(a1, a2, a3, a4);
            };
            let manager = engine.sound_object_manager();
            let sound_object = manager.sound_object(a4);
            let switch_name_wwise_id = engine.metadata_manager().switch_group_id(a2);
            let switch_value_wwise_id = engine.metadata_manager().switch_id(a3);
            let entity_id = sound_object.map(|x| x.entity_id()).unwrap_or_default();
            let emitter_name = sound_object.map(|x| x.emitter_name()).unwrap_or_default();
            let sound_tags = sound_object
                .map(|x| x.sound_tags().map(|x| x.to_vec()).unwrap_or_default())
                .unwrap_or_default();
            let emitter_tags = sound_object
                .map(|x| x.emitter_tags().map(|x| x.to_vec()).unwrap_or_default())
                .unwrap_or_default();
            if AudioEventCallbackSystem::any_callback(event_name, event_type) {
                AudioEventCallbackSystem::dispatch(FireCallback::SetSwitch(
                    FireSetSwitchCallback {
                        base: FirePlayCallback {
                            event_name,
                            event_type,
                            emitter_name,
                            entity_id,
                            sound_tags,
                            emitter_tags,
                            wwise_id: switch_name_wwise_id,
                        },
                        switch_name: a2,
                        switch_value: a3,
                        switch_name_wwise_id,
                        switch_value_wwise_id,
                    },
                ));
            }
            if !Replacements.is_specific_muted(event_name, event_type) {
                crate::utils::intercept!(
                    "SoundEngine::SetSwitch( {a2}, {a3}, {a4} ) / switch_name_wwise_id: {switch_name_wwise_id} switch_value_wwise_id: {switch_value_wwise_id}"
                );
                return cb(a1, a2, a3, a4);
            }
        }
        0
    }
}

pub mod event {
    use red4ext_rs::VoidPtr;

    use crate::{
        AudioEventCallbackSystem, AudioInternalEvent, EventActionType, EventApplicationInterface,
        EventName, SoundEngine,
        abi::callback::{
            FireAddContainerStreamingPrefetchCallback, FireCallback,
            FireRemoveContainerStreamingPrefetchCallback, FireSetAppearanceNameCallback,
            FireSetEntityNameCallback, FireStopCallback, FireStopTaggedCallback, FireTagCallback,
            FireUntagCallback,
        },
        engine::{Mute, Replacements},
    };

    ::red4ext_rs::hooks! {
        static HOOK: fn(a1: *const AudioInternalEvent,
        a2: VoidPtr,
        a3: VoidPtr) -> ();
    }

    #[allow(clippy::missing_transmute_annotations)]
    pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
        let addr = ::red4ext_rs::addr_hashes::resolve(
            super::super::offsets::AUDIOINTERNALEVENT_APPLY_ACTION,
        );
        let addr = unsafe { ::std::mem::transmute(addr) };
        unsafe { env.attach_hook(HOOK, addr, detour) };
        crate::utils::intercept!(
            "attached native internal hook for AudioInternalEvent::ApplyAction( VoidPtr, VoidPtr, VoidPtr )"
        );
    }

    unsafe extern "C" fn detour(
        a1: *const AudioInternalEvent,
        a2: VoidPtr,
        a3: VoidPtr,
        cb: unsafe extern "C" fn(a1: *const AudioInternalEvent, a2: VoidPtr, a3: VoidPtr) -> (),
    ) {
        unsafe {
            let audio = &*a1;
            let name = audio.event_name();
            let event_type = audio.event_type();

            const TYPES: &[EventActionType] = &[
                EventActionType::SetAppearanceName,
                EventActionType::SetEntityName,
                EventActionType::StopSound,
                EventActionType::StopTagged,
                EventActionType::AddContainerStreamingPrefetch,
                EventActionType::RemoveContainerStreamingPrefetch,
                EventActionType::Tag,
                EventActionType::Untag,
            ];
            if TYPES.contains(&event_type)
                && let Ok(event_name) = EventName::try_from(name)
            {
                let entity_id = EventApplicationInterface::new(a2).entity_id();
                let emitter_name = EventApplicationInterface::new(a2).emitter_name();
                let metadata_name = EventApplicationInterface::new(a2).metadata_name();
                let float_data = audio.float_data().unwrap_or(0.);
                let wwise_id = SoundEngine::get().metadata_manager().wwise_id(name);
                if AudioEventCallbackSystem::any_callback(event_name, event_type) {
                    match event_type {
                        EventActionType::SetAppearanceName => {
                            AudioEventCallbackSystem::dispatch(FireCallback::SetAppearanceName(
                                FireSetAppearanceNameCallback {
                                    event_name,
                                    entity_id,
                                    emitter_name,
                                    metadata_name,
                                    wwise_id,
                                },
                            ));
                        }
                        EventActionType::SetEntityName => {
                            AudioEventCallbackSystem::dispatch(FireCallback::SetEntityName(
                                FireSetEntityNameCallback {
                                    event_name,
                                    entity_id,
                                    emitter_name,
                                    metadata_name,
                                    wwise_id,
                                },
                            ));
                        }
                        EventActionType::StopSound => {
                            AudioEventCallbackSystem::dispatch(FireCallback::Stop(
                                FireStopCallback {
                                    event_name,
                                    entity_id,
                                    emitter_name,
                                    metadata_name,
                                    event_type,
                                    float_data,
                                    wwise_id,
                                },
                            ));
                        }
                        EventActionType::StopTagged => {
                            AudioEventCallbackSystem::dispatch(FireCallback::StopTagged(
                                FireStopTaggedCallback {
                                    event_name,
                                    entity_id,
                                    emitter_name,
                                    metadata_name,
                                    event_type,
                                    float_data,
                                    wwise_id,
                                },
                            ));
                        }
                        EventActionType::Tag => {
                            AudioEventCallbackSystem::dispatch(FireCallback::Tag(
                                FireTagCallback {
                                    event_name,
                                    entity_id,
                                    emitter_name,
                                    metadata_name,
                                    event_type,
                                    wwise_id,
                                },
                            ));
                        }
                        EventActionType::Untag => {
                            AudioEventCallbackSystem::dispatch(FireCallback::Untag(
                                FireUntagCallback {
                                    event_name,
                                    entity_id,
                                    emitter_name,
                                    metadata_name,
                                    event_type,
                                    wwise_id,
                                },
                            ));
                        }
                        EventActionType::AddContainerStreamingPrefetch => {
                            AudioEventCallbackSystem::dispatch(
                                FireCallback::AddContainerStreamingPrefetch(
                                    FireAddContainerStreamingPrefetchCallback {
                                        event_name,
                                        entity_id,
                                        emitter_name,
                                        metadata_name,
                                        event_type,
                                        wwise_id,
                                    },
                                ),
                            );
                        }
                        EventActionType::RemoveContainerStreamingPrefetch => {
                            AudioEventCallbackSystem::dispatch(
                                FireCallback::RemoveContainerStreamingPrefetch(
                                    FireRemoveContainerStreamingPrefetchCallback {
                                        event_name,
                                        entity_id,
                                        emitter_name,
                                        metadata_name,
                                        event_type,
                                        wwise_id,
                                    },
                                ),
                            );
                        }
                        _ => {}
                    };
                };
                if Replacements.is_specific_muted(event_name, event_type) {
                    return;
                }
                // crate::utils::intercept!(
                //     "AudioInternalEvent::ApplyAction( .. ) / event_name: {name}, event_type: {event_type}, entity_id: {entity_id}, emitter_name: {emitter_name}, metadata_name: {metadata_name}"
                // );
            }
            cb(a1, a2, a3);
        }
    }
}
