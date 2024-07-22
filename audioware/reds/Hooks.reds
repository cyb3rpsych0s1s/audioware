module Audioware

@wrapMethod(GameObject)
protected func Update(dt: Float) -> Void {
    wrappedMethod(dt);
    if this.IsPlayer() {
        let position: Vector4 = this.GetWorldPosition();
        let orientation: Quaternion = this.GetWorldOrientation();
        UpdateListener(position, orientation);
    } else {
        let id = this.GetEntityID();
        let position = this.GetWorldPosition();
        UpdateEmitter(id, position);
    }
}