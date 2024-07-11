module Audioware

private native func RegisterEmitter(emitterID: EntityID, emitterName: CName) -> Void;
private native func UnregisterEmitter(emitterID: EntityID) -> Void;