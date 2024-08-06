module Audioware

enum Easing {
    InPowf = 0,
    OutPowf = 1,
    InOutPowf = 2,
}

public abstract class Tween extends IScriptable {
    /// delay before starting: in seconds
    private let startTime: Float;
    /// tween duration: in seconds
    private let duration: Float;
    public func StartTime() -> Float { return this.startTime; }
    public func Duration() -> Float  { return this.duration;  }
}
public class LinearTween extends Tween {
    static public func Immediate(duration: Float) -> ref<LinearTween> {
        let me = new LinearTween();
        me.startTime = 0.;
        me.duration = duration;
        return me;
    }
}
public class ElasticTween extends Tween {
    /// tween curve
    private let easing: Easing;
    /// tween curve intensity
    private let value: Float;
    public func Easing() -> Easing { return this.easing; }
    public func Value() -> Float            { return this.value;  }
    static public func Immediate(duration: Float, value: Float, easing: Easing) -> ref<ElasticTween> {
        let me = new ElasticTween();
        me.startTime = 0.;
        me.duration = duration;
        me.easing = easing;
        me.value = value;
        return me;
    }
    static public func ImmediateIn(duration: Float, value: Float) -> ref<ElasticTween> {
        return ElasticTween.Immediate(duration, value, Easing.InPowf);
    }
    static public func ImmediateOut(duration: Float, value: Float) -> ref<ElasticTween> {
        return ElasticTween.Immediate(duration, value, Easing.OutPowf);
    }
    static public func ImmediateInOut(duration: Float, value: Float) -> ref<ElasticTween> {
        return ElasticTween.Immediate(duration, value, Easing.InOutPowf);
    }
}
