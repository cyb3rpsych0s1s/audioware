module Audioware

public class AudiowareConfig {
    @runtimeProperty("ModSettings.mod", "Audioware")
    @runtimeProperty("ModSettings.displayName", "Audio buffer size (optional)")
    @runtimeProperty("ModSettings.description", "Adjust audio buffer size (plays smoothlier, but adds latency): requires game restart")
    // NOTE: ModSettings does not seem to support RestartRequired
    @runtimeProperty("ModSettings.updatePolicy", "RestartRequired") // ConfigVarUpdatePolicy.RestartRequired = 3
    @runtimeProperty("ModSettings.displayValues.Auto", "Auto")
    @runtimeProperty("ModSettings.displayValues.Option64", "64 Samples")
    @runtimeProperty("ModSettings.displayValues.Option128", "128 Samples")
    @runtimeProperty("ModSettings.displayValues.Option256", "256 Samples")
    @runtimeProperty("ModSettings.displayValues.Option512", "512 Samples")
    @runtimeProperty("ModSettings.displayValues.Option1024", "1024 Samples")
    public let bufferSize: BufferSize = BufferSize.Auto;
}

/// NOTE: ModSettings enum variants do not play well with arbitrary values
enum BufferSize {
    Auto = 0,
    Option64 = 1,
    Option128 = 2,
    Option256 = 3,
    Option512 = 4,
    Option1024 = 5,
}
