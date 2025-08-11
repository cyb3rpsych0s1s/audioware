module Audioware

import Codeware.Localization.PlayerGender

/// import Audioware.HotReload
/// HotReload();
///
/// (note: this function does nothing in release build)
private native func HotReload();

private native func OnGameSessionBeforeStart();
private native func OnGameSessionStart();
private native func OnGameSessionReady();
private native func OnGameSessionPause();
private native func OnGameSessionResume();
private native func OnGameSessionBeforeEnd();
private native func OnGameSessionEnd();

private native func OnGameSystemAttach();
private native func OnGameSystemPlayerAttach();
private native func OnGameSystemPlayerDetach();
private native func OnGameSystemDetach();

private native func OnUIMenu(value: Bool);
private native func OnEngagementScreen();

private native func SetVolume(setting: CName, value: Float);
private native func SetMuteInBackground(value: Bool);
private native func SetReverbMix(value: Float) -> Void;
private native func SetPreset(value: Preset) -> Void;
private native func SetPlayerGender(value: PlayerGender) -> Void;
private native func UnsetPlayerGender() -> Void;
private native func SetGameLocales(spoken: CName, written: CName) -> Void;

public enum LocaleExt {
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