module Audioware

@addMethod(AudioSystem)
public func IsRegisteredEmitter(entityID: EntityID) -> Bool {
    return IsRegisteredEmitter(entityID);
}

@addMethod(AudioSystem)
public func RegisterEmitter(entityID: EntityID, opt emitterName: CName) -> Void {
    RegisterEmitter(entityID, emitterName);
    AudiowareService.GetInstance().AddTarget(EntityTarget.ID(entityID));
}

@addMethod(AudioSystem)
public func UnregisterEmitter(entityID: EntityID) -> Void {
    UnregisterEmitter(entityID);
    AudiowareService.GetInstance().RemoveTarget(EntityTarget.ID(entityID));
}

@addMethod(AudioSystem)
public func EmittersCount() -> Int32 = EmittersCount();

@addMethod(AudioSystem)
public func PlayOverThePhone(eventName: CName, emitterName: CName, gender: CName) -> Void {
    PlayOverThePhone(eventName, emitterName, gender);
}

@addMethod(AudioSystem)
public func Stop(eventName: CName, opt entityID: EntityID, opt emitterName: CName, tween: ref<AudiowareTween>) -> Void {
    Stop(eventName, entityID, emitterName, tween);
}
