module Audioware

/// necessary evil
private func Play(eventName: CName, entityID: EntityID, emitterName: CName) -> Void {
    GameInstance.GetAudioSystem(GetGameInstance()).Play(eventName, entityID, emitterName);
}
private func Stop(eventName: CName, entityID: EntityID, emitterName: CName) -> Void {
    GameInstance.GetAudioSystem(GetGameInstance()).Stop(eventName, entityID, emitterName);
}
private func Switch(eventName: CName, eventValue: CName, entityID: EntityID, emitterName: CName) -> Void {
    GameInstance.GetAudioSystem(GetGameInstance()).Switch(eventName, eventValue, entityID, emitterName);
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
