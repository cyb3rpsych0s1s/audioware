module Audioware

public native struct LoopRegion {
    // in seconds
    public let starts: Float;
    // in seconds
    public let ends: Float;
}

public native struct Args {
    /// in seconds
    private let startPosition: Float;
    /// factor
    private let volume: Float;
    /// between 0.0 and 1.0 (inclusive)
    private let panning: Float;
    /// factor
    private let playbackRate: Float;
    private let loopRegion: LoopRegion;
    private let fadeInTween: ref<Tween>;
    public static final func Default() -> Args {
        return new Args(0.0, 1.0, 0.5, 1.0, new LoopRegion(0.0, 0.0), null);
    }
    public static final func SetStartPosition(self: Args, value: Float) -> Args {
        self.startPosition = value;
        return self;
    }
    public static final func SetVolume(self: Args, value: Float) -> Args {
        self.volume = value;
        return self;
    }
    public static final func SetPanning(self: Args, value: Float) -> Args {
        self.panning = value;
        return self;
    }
    public static final func SetPlaybackRate(self: Args, value: Float) -> Args {
        self.playbackRate = value;
        return self;
    }
    public static final func SetLoopRegion(self: Args, starts: Float, ends: Float) -> Args {
        self.loopRegion.starts = starts;
        self.loopRegion.ends = ends;
        return self;
    }
}