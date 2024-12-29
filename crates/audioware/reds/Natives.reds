module Audioware

import Codeware.Localization.PlayerGender

/// import Audioware.HotReload
/// HotReload();
///
/// (note: this function does nothing in release build)
private static native func HotReload();

private static native func OnGameSessionBeforeStart();
private static native func OnGameSessionStart();
private static native func OnGameSessionReady();
private static native func OnGameSessionPause();
private static native func OnGameSessionResume();
private static native func OnGameSessionBeforeEnd();
private static native func OnGameSessionEnd();

private static native func OnGameSystemAttach();
private static native func OnGameSystemPlayerAttach();
private static native func OnGameSystemPlayerDetach();
private static native func OnGameSystemDetach();

private static native func OnUIMenu(value: Bool);

private static native func SetVolume(setting: CName, value: Double);
private static native func SetReverbMix(value: Float) -> Void;
private static native func SetPreset(value: Preset) -> Void;
private static native func SetPlayerGender(value: PlayerGender) -> Void;
private static native func UnsetPlayerGender() -> Void;
private static native func SetGameLocales(spoken: CName, written: CName) -> Void;

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