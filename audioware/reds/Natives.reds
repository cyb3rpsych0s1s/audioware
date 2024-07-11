module Audioware

private native func RegisterListener(listenerID: EntityID) -> Void;
private native func UnregisterListener(listenerID: EntityID) -> Void;
private native func RegisterEmitter(emitterID: EntityID, emitterName: CName) -> Void;
private native func UnregisterEmitter(emitterID: EntityID) -> Void;