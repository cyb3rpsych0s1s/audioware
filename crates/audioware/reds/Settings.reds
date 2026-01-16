module Audioware

public class AudioRegion {
    public let starts: Float = 0.;
    public let ends: Float = 0.;
}

public class AudioSettingsExt {
    public let startPosition: Float = 0.;
    public let region: ref<AudioRegion>;
    public let loop: Bool = false;
    public let volume: Float = 1.;
    public let fadeIn: ref<Tween>;
    public let panning: Float = 0.5;
    public let playbackRate: Float = 1.;
    public let affectedByTimeDilation: Bool = true;
}

public class EmitterDistances {
    let min: Float = 0.0;
    let max: Float = 0.0;
}

public class EmitterSettings {
    public let distances: ref<EmitterDistances>;
    public let attenuationFunction: ref<Tween>;
    public let enableSpatialization: Bool = true;
    public let persistUntilSoundsFinish: Bool = false;
    public let affectedByReverbMix: Bool = true;
    public let affectedByEnvironmentalPreset: Bool = false;
    public let enableOcclusion: Bool = false;
}
