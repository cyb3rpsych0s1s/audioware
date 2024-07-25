module Audioware

import Codeware.Localization.PlayerGender

private static func LOG(msg: String) {
    FTLog(AsRef(msg));
    PLog(msg);
}
private static func WARN(msg: String) {
    FTLogWarning(AsRef(msg));
    PLogWarning(msg);
}
private static func ERR(msg: String) {
    FTLogError(AsRef(msg));
    PLogError(msg);
}

private native func PLog(msg: String) -> Void;
private native func PLogWarning(msg: String) -> Void;
private native func PLogError(msg: String) -> Void;

private native func RegisterEmitter(emitterID: EntityID, opt emitterName: CName) -> Void;
private native func UnregisterEmitter(emitterID: EntityID) -> Void;
private native func EmittersCount() -> Int32;
private native func ClearEmitters() -> Void;

private native func SetGameState(state: GameState) -> Void;
private native func SetPlayerGender(gender: PlayerGender) -> Void;
private native func UnsetPlayerGender() -> Void;
private native func SetGameLocales(spoken: CName, written: CName) -> Void;

private native func Play(eventName: CName, opt entityID: EntityID, opt emitterName: CName, tween: ref<AudiowareTween>) -> Void;
private native func Stop(eventName: CName, opt entityID: EntityID, opt emitterName: CName, tween: ref<AudiowareTween>) -> Void;
private native func Switch(switchName: CName, switchValue: CName, opt entityID: EntityID, opt emitterName: CName, switchNameTween: ref<AudiowareTween>, switchValueTween: ref<AudiowareTween>) -> Void;

private native func PlayOnEmitter(eventName: CName, entityID: EntityID, emitterName: CName, tween: ref<AudiowareTween>) -> Void;
private native func StopOnEmitter(eventName: CName, entityID: EntityID, emitterName: CName, tween: ref<AudiowareTween>) -> Void;

enum GameState {
    Load = 0,
    Menu = 1,
    Start = 2,
    InGame = 3,
    InMenu = 4,
    InPause = 5,
    End = 6,
    Unload = 7,
}
