module Audioware

import Codeware.Localization.PlayerGender

private native func RegisterListener(listenerID: EntityID) -> Void;
private native func UnregisterListener(listenerID: EntityID) -> Void;
private native func RegisterEmitter(emitterID: EntityID, emitterName: CName) -> Void;
private native func UnregisterEmitter(emitterID: EntityID) -> Void;
private native func EmittersCount() -> Int32;
private native func SetGameState(state: GameState) -> Void;
private native func SetPlayerGender(gender: PlayerGender) -> Void;
private native func UnsetPlayerGender() -> Void;
private native func SetGameLocales(spoken: CName, written: CName) -> Void;

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
