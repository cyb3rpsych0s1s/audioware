module Audioware

private native func StopEngine() -> Void;

private native func AudiowarePlayOnTrack(eventName: CName, trackName: CName, entityID: EntityID, emitterName: CName, tween: ref<AudiowareTween>) -> Void;
private native func AudiowareTrackStop(eventName: CName, entityID: EntityID, emitterName: CName, tween: ref<AudiowareTween>) -> Void;
private native func AudiowareAddTrack(trackName: CName) -> Void;
private native func AudiowareRemoveTrack(trackName: CName) -> Void;

enum EngineState {
    Load = 0,
    Menu = 1,
    Start = 2,
    InGame = 3,
    InMenu = 4,
    InPause = 5,
    End = 6,
    Unload = 7,
}

private native func UpdateGameState(state: EngineState) -> Void;

public static func FindEntityByID(gi: GameInstance, id: EntityID) -> ref<Entity> {
    return GameInstance.FindEntityByID(gi, id);
}

public static func DelegatePlay(eventName: CName, opt entityID: EntityID, opt emitterName: CName) -> Void {
    GameInstance.GetAudioSystem(GetGameInstance()).Play(eventName, entityID, emitterName);
}
public static func DelegateStop(eventName: CName, opt entityID: EntityID, opt emitterName: CName) -> Void {
    GameInstance.GetAudioSystem(GetGameInstance()).Stop(eventName, entityID, emitterName);
}
