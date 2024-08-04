public native struct OneStruct {
    let value: Float;
}

public native func TestOneStruct(s: OneStruct) -> Void;

/// Game.CallTestOneStruct();
public static exec func CallTestOneStruct(game: GameInstance) {
    let o = new OneStruct(2077.0);
    TestOneStruct(o);
}