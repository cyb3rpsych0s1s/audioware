@wrapMethod(GameObject)
public final static func PlaySoundEvent(self: ref<GameObject>, eventName: CName) -> Void {
    if self.IsExactlyA(n"PlayerPuppet") { LogChannel(n"DEBUG", "is a player puppet"); }
    wrappedMethod(self, eventName);
}

@wrapMethod(GameObject)
public final static func StopSoundEvent(self: ref<GameObject>, eventName: CName) -> Void {
    if self.IsExactlyA(n"PlayerPuppet") { LogChannel(n"DEBUG", "is a player puppet"); }
    wrappedMethod(self, eventName);
}

// detour ?
// public native class Entity extends IScriptable {
//     public final native func QueueEvent(evt: ref<Event>) -> Void;
//     public final native func QueueEventForEntityID(entityID: EntityID, evt: ref<Event>) -> Bool;
// }