module Audioware

import Codeware.Localization.PlayerGender

private native func RegisterListener(listenerID: EntityID) -> Void;
private native func UnregisterListener(listenerID: EntityID) -> Void;
private native func RegisterEmitter(emitterID: EntityID, emitterName: CName) -> Void;
private native func UnregisterEmitter(emitterID: EntityID) -> Void;
private native func EmittersCount() -> Int32;
private native func SetPlayerGender(gender: PlayerGender) -> Void;
private native func UnsetPlayerGender() -> Void;
