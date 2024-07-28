enum AudiowareEasing {
    InPowf = 0,
    OutPowf = 1,
    InOutPowf = 2,
}

public abstract class AudiowareTween extends IScriptable {
    /// delay before starting: in seconds
    public let startTime: Float;
    /// tween duration: in seconds
    public let duration: Float;
    public func StartTime() -> Float { return this.startTime; }
    public func Duration() -> Float  { return this.duration;  }
}
public class AudiowareLinearTween extends AudiowareTween {
    static public func Immediate(duration: Float) -> ref<AudiowareLinearTween> {
        let me = new AudiowareLinearTween();
        me.startTime = 0.;
        me.duration = duration;
        return me;
    }
}
public class AudiowareElasticTween extends AudiowareTween {
    /// tween curve
    public let easing: AudiowareEasing;
    /// tween curve intensity
    public let value: Float;
    public func Easing() -> AudiowareEasing { return this.easing; }
    public func Value() -> Float            { return this.value;  }
    static public func Immediate(duration: Float, value: Float, easing: AudiowareEasing) -> ref<AudiowareElasticTween> {
        let me = new AudiowareElasticTween();
        me.startTime = 0.;
        me.duration = duration;
        me.easing = easing;
        me.value = value;
        return me;
    }
    static public func ImmediateIn(duration: Float, value: Float) -> ref<AudiowareElasticTween> {
        let me = AudiowareElasticTween.Immediate(duration, value, AudiowareEasing.InPowf);
        return me;
    }
    static public func ImmediateOut(duration: Float, value: Float) -> ref<AudiowareElasticTween> {
        let me = AudiowareElasticTween.Immediate(duration, value, AudiowareEasing.OutPowf);
        return me;
    }
    static public func ImmediateInOut(duration: Float, value: Float) -> ref<AudiowareElasticTween> {
        let me = AudiowareElasticTween.Immediate(duration, value, AudiowareEasing.InOutPowf);
        return me;
    }
}