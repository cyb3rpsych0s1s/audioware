module Audioware

enum Easing {
    InPowf = 0,
    OutPowf = 1,
    InOutPowf = 2,
}

public abstract class Tween extends IScriptable {
    /// delay before starting: in seconds
    public let startTime: Float;
    /// tween duration: in seconds
    public let duration: Float;
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
    public let easing: Easing;
    /// tween curve intensity
    public let value: Float;
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
