use red4ext_rs::{
    addr_hashes, hooks, log,
    types::{IScriptable, StackFrame},
    PluginOps, SdkEnv, VoidPtr,
};

use crate::{
    types::{AsEntity, AsIComponent, IComponent, Quaternion},
    Audioware,
};

hooks! {
   static HOOK: fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr) -> ();
}

#[allow(clippy::missing_transmute_annotations)]
pub fn attach_hook(env: &SdkEnv) {
    let addr = addr_hashes::resolve(super::offsets::SET_LOCAL_ORIENTATION);
    let addr = unsafe { std::mem::transmute(addr) };
    unsafe { env.attach_hook(HOOK, addr, detour) };
    log::info!(
        env,
        "attached hook for IPlacedComponent.SetLocalOrientation"
    );
}

#[allow(unused_variables)]
unsafe extern "C" fn detour(
    i: *mut IScriptable,
    f: *mut StackFrame,
    a3: VoidPtr,
    a4: VoidPtr,
    cb: unsafe extern "C" fn(i: *mut IScriptable, f: *mut StackFrame, a3: VoidPtr, a4: VoidPtr),
) {
    let frame = &mut *f;
    let state = frame.args_state();

    let rot: Quaternion = StackFrame::get_arg(frame);

    let env = Audioware::env();
    if !i.is_null() {
        if let Some(icomponent) = (*i)
            .as_serializable()
            .inner_ref::<IComponent>()
            .and_then(|x| x.upgrade())
        {
            if let Some(entity) = icomponent.get_entity().upgrade() {
                let entity_id = entity.get_entity_id();
                log::info!(
                    env,
                    "IPlacedComponent.SetLocalOrientation: EntityID {entity_id:?}"
                );
            }
        }
    }
    log::info!(
        env,
        "IPlacedComponent.SetLocalOrientation: intercepted {rot}"
    );

    frame.restore_args(state);
    cb(i, f, a3, a4);
}
