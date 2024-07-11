module Audioware

@addMethod(AudioSystem)
public func RegisterEmitter(entityID: EntityID, emitterName: CName) -> Void { RegisterEmitter(entityID, emitterName); }

@addMethod(AudioSystem)
public func UnregisterEmitter(entityID: EntityID) -> Void { UnregisterEmitter(entityID); }

@addMethod(AudioSystem)
public func EmittersCount() -> Int32 = EmittersCount();
