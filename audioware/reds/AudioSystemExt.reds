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
public func Play(eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt line: scnDialogLineType, tween: ref<AudiowareTween>) -> Void {
    Play(eventName, entityID, emitterName, line, tween);
}

@addMethod(AudioSystem)
public func Stop(eventName: CName, opt entityID: EntityID, opt emitterName: CName, tween: ref<AudiowareTween>) -> Void {
    Stop(eventName, entityID, emitterName, tween);
}

@addMethod(AudioSystem)
public func Switch(switchName: CName, switchValue: CName, opt entityID: EntityID, opt emitterName: CName, opt switchNameTween: ref<AudiowareTween>, opt switchValueTween: ref<AudiowareTween>) -> Void {
    Switch(switchName, switchValue, entityID, emitterName, switchNameTween, switchValueTween);
}

@addMethod(AudioSystem)
public func PlayOnEmitter(eventName: CName, entityID: EntityID, emitterName: CName, opt tween: ref<AudiowareTween>) -> Void {
    PlayOnEmitter(eventName, entityID, emitterName, tween);
}

@addMethod(AudioSystem)
public func StopOnEmitter(eventName: CName, entityID: EntityID, emitterName: CName, opt tween: ref<AudiowareTween>) -> Void {
    StopOnEmitter(eventName, entityID, emitterName, tween);
}
