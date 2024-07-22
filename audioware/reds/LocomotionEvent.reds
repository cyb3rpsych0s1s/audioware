module Audioware

@wrapMethod(ClimbEvents)
public func OnUpdate(timeDelta: Float, stateContext: ref<StateContext>, scriptInterface: ref<StateGameScriptInterface>) -> Void {
    wrappedMethod(timeDelta, stateContext, scriptInterface);
    let position: Vector4 = scriptInterface.owner.GetWorldPosition();
    let orientation: Quaternion = scriptInterface.owner.GetWorldOrientation();
    UpdateListener(position, orientation);
}