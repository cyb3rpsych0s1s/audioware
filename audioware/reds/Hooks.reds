module Audioware

@addField(LocomotionEventsTransition)
private let lastDelta: Float;

@wrapMethod(LocomotionEventsTransition)
public func OnUpdate(timeDelta: Float, stateContext: ref<StateContext>, scriptInterface: ref<StateGameScriptInterface>) -> Void {
    wrappedMethod(timeDelta, stateContext, scriptInterface);
    if timeDelta < (this.lastDelta + 0.2) { return; }
    this.lastDelta = timeDelta;
    let me = scriptInterface.executionOwner;
    let position: Vector4 = me.GetWorldForward();
    let orientation: Quaternion = me.GetWorldOrientation();
    UpdateListener(position, orientation);
}

@addField(GameObject)
private let lastDelta: Float;

@wrapMethod(GameObject)
protected func OnTransformUpdated() -> Void {
    wrappedMethod();
    let now = GameInstance.GetTimeSystem(this.GetGame()).GetGameTimeStamp();
    if now > (this.lastDelta + 0.2) {
        this.lastDelta = now;
        let me = this.GetEntity();
        let me = EntityGameInterface.GetEntity(me);
        if IsDefined(me) {
            let id = me.GetEntityID();
            let position = me.GetWorldPosition();
            UpdateEmitter(id, position);
        }
    }
}

// @wrapMethod(GameObject)
// public final func PassUpdate(dt: Float) -> Void {
//     wrappedMethod(dt);
//     LOG(s"🆕 GameObject.PassUpdate");
// }

// @wrapMethod(GameObject)
// protected func Update(dt: Float) -> Void {
//     wrappedMethod(dt);
//     LOG(s"🆕 GameObject.Update");
// }

// @wrapMethod(UpdateComponent)
// public final func OnUpdate(deltaTime: Float) -> Void {
//     wrappedMethod(deltaTime);
// }