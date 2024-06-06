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
public class AudiowareLinearTween extends AudiowareTween {
    static public func Immediate(duration: Uint32) -> ref<AudiowareLinearTween> {
        let me = new AudiowareLinearTween();
        me.startTime = 0u;
        me.duration = duration;
        return me;
    }
}
public class AudiowareElasticTween extends AudiowareTween {
    public let easing: AudiowareEasing;
    public let value: Int32;
    public func Easing() -> AudiowareEasing { return this.easing; }
    public func Value() -> Int32            { return this.value;  }
    static public func Immediate(duration: Uint32, value: Int32, easing: AudiowareEasing) -> ref<AudiowareElasticTween> {
        let me = new AudiowareElasticTween();
        me.startTime = 0u;
        me.duration = duration;
        me.easing = easing;
        me.value = value;
        return me;
    }
    static public func ImmediateIn(duration: Uint32, value: Int32) -> ref<AudiowareElasticTween> {
        let me = AudiowareElasticTween.Immediate(duration, value, AudiowareEasing.InPowi);
        return me;
    }
    static public func ImmediateOut(duration: Uint32, value: Int32) -> ref<AudiowareElasticTween> {
        let me = AudiowareElasticTween.Immediate(duration, value, AudiowareEasing.OutPowi);
        return me;
    }
    static public func ImmediateInOut(duration: Uint32, value: Int32) -> ref<AudiowareElasticTween> {
        let me = AudiowareElasticTween.Immediate(duration, value, AudiowareEasing.InOutPowi);
        return me;
    }
}
