module Audioware

public class AudioSettingsExt {
    public let startPosition: Float;
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