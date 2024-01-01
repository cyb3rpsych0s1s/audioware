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
}

private native func UpdateEngineState(state: EngineState) -> Void;
