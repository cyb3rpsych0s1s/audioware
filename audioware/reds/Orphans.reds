enum AudiowareEasing {
    InPowi = 0,
    OutPowi = 1,
    InOutPowi = 2,
}

public abstract class AudiowareTween extends IScriptable {
    public let startTime: Uint32;
    public let duration: Uint32;
    public func StartTime() -> Uint32 { return this.startTime; }
    public func Duration() -> Uint32  { return this.duration;  }
}
public class AudiowareLinearTween extends AudiowareTween {}
public class AudiowareElasticTween extends AudiowareTween {
    public let easing: AudiowareEasing;
    public let value: Int32;
    public func Easing() -> AudiowareEasing { return this.easing; }
    public func Value() -> Int32            { return this.value;  }
}
