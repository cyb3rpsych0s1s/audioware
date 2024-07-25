module Audioware

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
