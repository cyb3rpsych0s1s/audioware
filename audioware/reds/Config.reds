enum BufferSize {
    Auto = 0,
    Option64 = 64,
    Option128 = 128,
    Option256 = 256,
    Option512 = 512,
    Option1024 = 1024,
}

public class AudiowareConfig {
  @runtimeProperty("ModSettings.mod", "Audioware")
  @runtimeProperty("ModSettings.displayName", "Mod-Audioware-BufferSize")
  @runtimeProperty("ModSettings.description", "Adjust audio buffer size (plays smoothlier, but adds latency): requires game restart")
  @runtimeProperty("ModSettings.displayValues.OptionAuto", "auto")
  @runtimeProperty("ModSettings.displayValues.Option32", "32 samples")
  @runtimeProperty("ModSettings.displayValues.Option64", "64 samples")
  @runtimeProperty("ModSettings.displayValues.Option128", "128 samples")
  @runtimeProperty("ModSettings.displayValues.Option256", "256 samples")
  @runtimeProperty("ModSettings.displayValues.Option512", "512 samples")
  @runtimeProperty("ModSettings.displayValues.Option1024", "1024 samples")
  public let bufferSize: BufferSize = BufferSize.Auto;
}