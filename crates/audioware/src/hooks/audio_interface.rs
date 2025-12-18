use red4ext_rs::VoidPtr;

use crate::{AudioEventId, AudioInternalEvent};

::red4ext_rs::hooks! {
    static HOOK: fn(a1: VoidPtr,
    a2: *mut AudioInternalEvent,
    a3: *mut AudioEventId) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &::red4ext_rs::SdkEnv) {
    let addr = ::red4ext_rs::addr_hashes::resolve(super::offsets::AUDIOINTERFACE_POST_EVENT);
    let addr = unsafe { ::std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    crate::utils::intercept!(
        "attached native internal hook for AudioInterface::PostEvent( AudioInternalEvent, AudioEventId )"
    );
}

unsafe extern "C" fn detour(
    a1: VoidPtr,
    a2: *mut AudioInternalEvent,
    a3: *mut AudioEventId,
    cb: unsafe extern "C" fn(a1: VoidPtr, a2: *mut AudioInternalEvent, a3: *mut AudioEventId) -> (),
) {
    unsafe {
        // if !a2.is_null() && !a1.offset(8).is_null() {
        //     let event = &*a2;
        //     let event_type = event.event_type();
        //     if ![
        //         EventActionType::Play,
        //         EventActionType::PlayAnimation,
        //         EventActionType::PlayExternal,
        //         EventActionType::SetParameter,
        //         EventActionType::SetSwitch,
        //     ]
        //     .contains(&event_type)
        //         && let Ok(event_name) = EventName::try_from(event.event_name())
        //     {
        //         if AudioEventCallbackSystem::any_callback(event_name, Some(event_type)) {
        //             match event_type {
        //                 EventActionType::AddContainerStreamingPrefetch => {
        //                     queue::forward(Callback::FireCallbacks(
        //                         FireCallback::AddContainerStreamingPrefetch(
        //                             FireAddContainerStreamingPrefetchCallback {
        //                                 event_name,
        //                                 event_type,
        //                                 entity_id: EntityId::default(),
        //                                 emitter_name: CName::default(),
        //                                 metadata_name: CName::default(),
        //                             },
        //                         ),
        //                     ));
        //                 }
        //                 EventActionType::RemoveContainerStreamingPrefetch => {
        //                     queue::forward(Callback::FireCallbacks(
        //                         FireCallback::RemoveContainerStreamingPrefetch(
        //                             FireRemoveContainerStreamingPrefetchCallback {
        //                                 event_name,
        //                                 event_type,
        //                                 entity_id: EntityId::default(),
        //                                 emitter_name: CName::default(),
        //                                 metadata_name: CName::default(),
        //                             },
        //                         ),
        //                     ));
        //                 }
        //                 _ => {} // TODO
        //             }
        //         }
        //         if !Replacements.is_specific_muted(event_name, event_type) {
        //             // crate::utils::intercept!("AudioInterface::PostEvent( {{ {event} }}, .. )");
        //             cb(a1, a2, a3);
        //         }
        //     } else {
        //         cb(a1, a2, a3);
        //     }
        // }
        cb(a1, a2, a3);
    }
}
