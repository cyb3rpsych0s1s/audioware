public native struct OneStruct {
    let value: Float;
    public native static func SetValue(me: OneStruct, value: Float) -> Void;
}

public native func TestOneStruct(s: OneStruct) -> Void;

/// Game.CallTestOneStruct();
public static exec func CallTestOneStruct(game: GameInstance) {
    let o = new OneStruct(2077.0);
    TestOneStruct(o);
    o.SetValue(2020.0);
    
    let a = new MyNativeClass();
    a.TestMyNativeClass(2077);
}

public native class MyNativeClass extends IScriptable {
    public native func TestMyNativeClass(value: Int32) -> Void;
}
