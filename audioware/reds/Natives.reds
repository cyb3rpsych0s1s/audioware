module Audioware

import Codeware.Localization.PlayerGender

/// debug or release build ?
public native func IsDebug() -> Bool;

private static func DBG(msg: String) {
    if IsDebug() {
        let prefixed = s"[Audioware] \(msg)";
        FTLog(AsRef(prefixed));
        PLog(msg);
    }
}
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

private native func Shutdown() -> Void;

private native func RegisterEmitter(emitterID: EntityID, opt emitterName: CName, opt emitterSettings: EmitterSettings) -> Bool;
private native func UnregisterEmitter(emitterID: EntityID) -> Bool;

private native func SetGameState(state: GameState) -> Void;
private native func SetPlayerGender(gender: PlayerGender) -> Void;
private native func UnsetPlayerGender() -> Void;
private native func SetGameLocales(spoken: CName, written: CName) -> Void;

private native func Pause(opt tween: ref<Tween>) -> Void;
private native func Resume(opt tween: ref<Tween>) -> Void;

private native func SetReverbMix(value: Float) -> Void;
private native func SetPreset(value: Preset) -> Void;
private native func SetVolume(setting: CName, value: Double) -> Void;

private native func AddEvents(eventsMetadata: ref<audioAudioEventArray>) -> Void;

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

enum LocaleExt {
    English = 0,
    Creole = 1,
    Japanese = 2,
    Arabic = 3,
    Russian = 4,
    SimplifiedChinese = 5,
    BrazilianPortuguese = 6,
    Swahili = 7,
    French = 8,
    Polish = 9,
    Spanish = 10,
    Italian = 11,
    German = 12,
    LatinAmericanSpanish = 13,
    Korean = 14,
    TraditionalChinese = 15,
    Czech = 16,
    Hungarian = 17,
    Turkish = 18,
    Thai = 19,
}
