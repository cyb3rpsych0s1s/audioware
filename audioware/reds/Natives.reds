module Audioware

enum EngineState {
    Load = 0,
    Menu = 1,
    Start = 2,
    InGame = 3,
    InMenu = 4,
    InPause = 5,
    End = 6,
    Unload = 7,
    Unreachable = 8, // special state to indicate internal error: should not be set from .reds
}

private native func UpdateEngineState(state: EngineState) -> Void;

public static func FindEntityByID(gi: GameInstance, id: EntityID) -> ref<Entity> {
    return GameInstance.FindEntityByID(gi, id);
}
