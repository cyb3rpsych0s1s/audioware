module Audioware

private native func StopEngine() -> Void;

private native func SmoothStop(eventName: CName, entityID: EntityID, emitterName: CName, tween: ref<AudiowareTween>) -> Void;

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
