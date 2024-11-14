module Audioware

public class AudioRegion {
    public let starts: Float = 0.;
    public let ends: Float = 0.;
}

public class AudioSettingsExt {
    public let startPosition: Float = 0.;
    public let region: ref<AudioRegion>;
    public let loop: Bool = false;
    public let volume: Float = 100.;
    public let fadeIn: ref<Tween>;
    public let panning: Float = 0.5;
    public let playbackRate: Float = 1.;
}

public native struct EmitterDistances {
    let min: Float = 1.0;
    let max: Float = 100.0;
}

public native struct EmitterSettings {
    let distances: EmitterDistances;
    let attenuationFunction: ref<Tween>;
    let enableSpatialization: Bool = true;
    let persistUntilSoundsFinish: Bool = false;
}