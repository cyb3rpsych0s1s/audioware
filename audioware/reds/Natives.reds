module Audioware

import Codeware.Localization.PlayerGender

public static func LOG(msg: String) {
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

private native func Shutdown() -> Void;

private native func RegisterEmitter(emitterID: EntityID, opt emitterName: CName, opt emitterSettings: EmitterSettings) -> Bool;
private native func UnregisterEmitter(emitterID: EntityID) -> Bool;
private native func EmittersCount() -> Int32;
private native func IsRegisteredEmitter(entityID: EntityID) -> Bool;

private native func SetGameState(state: GameState) -> Void;
private native func SetPlayerGender(gender: PlayerGender) -> Void;
private native func UnsetPlayerGender() -> Void;
private native func SetGameLocales(spoken: CName, written: CName) -> Void;

private native func SetReverbMix(value: Float) -> Void;
private native func SetPreset(value: Preset) -> Void;

public native func PlayOverThePhone(eventName: CName, emitterName: CName, gender: CName) -> Void;
public native func Play(eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt line: scnDialogLineType, opt tween: ref<Tween>) -> Void;
public native func Stop(eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt tween: ref<Tween>) -> Void;
public native func Switch(switchName: CName, switchValue: CName, opt entityID: EntityID, opt emitterName: CName, opt switchNameTween: ref<Tween>, opt switchValueTween: ref<Tween>) -> Void;
public native func Pause(opt tween: ref<Tween>) -> Void;
public native func Resume(opt tween: ref<Tween>) -> Void;

public native func PlayOnEmitter(eventName: CName, entityID: EntityID, emitterName: CName, opt tween: ref<Tween>) -> Void;
public native func StopOnEmitter(eventName: CName, entityID: EntityID, emitterName: CName, opt tween: ref<Tween>) -> Void;

private native func SetVolume(setting: CName, value: Double) -> Void;

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
