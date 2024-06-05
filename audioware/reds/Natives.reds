module Audioware

private native func StopEngine() -> Void;

private native func StopLinear(eventName: CName, entityID: EntityID, emitterName: CName, tween: LinearTween) -> Void;
private native func StopElastic(eventName: CName, entityID: EntityID, emitterName: CName, tween: ElasticTween) -> Void;

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

enum Easing {
    InPowi = 0,
    OutPowi = 1,
    InOutPowi = 2,
}

public struct LinearTween {
    public let startTime: Uint32;
    public let duration: Uint32;
}

public struct ElasticTween {
    public let startTime: Uint32;
    public let duration: Uint32;
    public let easing: Easing;
}
