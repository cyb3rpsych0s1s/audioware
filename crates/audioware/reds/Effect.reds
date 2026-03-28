module Audioware

public abstract native class DynamicEffect {
  public native func Active() -> Bool;
}

enum EqFilterKind {
  Bell = 0,
  LowShelf = 1,
  HighShelf = 2,
}

public native class DynamicEQ extends DynamicEffect {
  // Sets the shape of the frequency adjustment curve.
  public native func SetKind(kind: EqFilterKind);

  // Sets the “center” or “corner” of the frequency range to adjust in Hz (for bell or shelf curves, respectively).
  public native func SetFrequency(frequency: Float, opt tween: ref<Tween>);

  // Sets the volume adjustment for frequencies in the specified range (in decibels).
  public native func SetGain(gain: Float, opt tween: ref<Tween>);

  // Sets the width of the frequency range to adjust.
  // A higher Q value results in a narrower range of frequencies being adjusted. The value should be greater than 0.0.
  public native func SetQ(q: Float, opt tween: ref<Tween>);

  public native static func Create(kind: EqFilterKind, frequency: Float, gain: Float, q: Float) -> ref<DynamicEQ>;
}

enum DistortionKind {
  HardClip = 0,
  SoftClip = 1,
}

public native class DynamicDistortion extends DynamicEffect {
  // Sets the kind of distortion to use.
  public native func SetKind(kind: DistortionKind);

  // Sets how much distortion should be applied (in decibels).
  public native func SetDrive(drive: Float, opt tween: ref<Tween>);

  // Sets how much dry (unprocessed) signal should be blended with the wet (processed) signal.
  // Valid mix values range from 0.0 to 1.0, where 0.0 is the dry signal only, 1.0 is the wet signal only, and 0.5 is an equal mix of both.
  public native func SetMix(mix: Float, opt tween: ref<Tween>);

  public native static func Create(kind: DistortionKind, drive: Float, mix: Float) -> ref<DynamicDistortion>;
}

public native class DynamicDelay extends DynamicEffect {
  // Sets the amount of feedback (in decibels).
  public native func SetFeedback(feedback: Float, opt tween: ref<Tween>);

  // Sets how much dry (unprocessed) signal should be blended with the wet (processed) signal.
  // Valid mix values range from 0.0 to 1.0, where 0.0 is the dry signal only, 1.0 is the wet signal only, and 0.5 is an equal mix of both.
  public native func SetMix(mix: Float, opt tween: ref<Tween>);

  public native static func Create(feedback: Float, mix: Float) -> ref<DynamicDelay>;
}

public native class DynamicCompressor extends DynamicEffect {
  // Sets the volume above which volume will start to be decreased (in decibels).
  public native func SetThreshold(threshold: Float, opt tween: ref<Tween>);

  // Sets how much the signal will be compressed.
  // A ratio of 2.0 (or 2 to 1) means an increase of 3dB will become an increase of 1.5dB. Ratios between 0.0 and 1.0 will actually expand the audio.
  public native func SetRatio(ratio: Float, opt tween: ref<Tween>);

  // Sets how much time it takes for the volume attenuation to ramp up once the input volume exceeds the threshold (in seconds).
  public native func SetAttackDuration(attackDuration: Float, opt tween: ref<Tween>);

  // Sets how much time it takes for the volume attenuation to relax once the input volume dips below the threshold (in seconds).
  public native func SetReleaseDuration(releaseDuration: Float, opt tween: ref<Tween>);

  // Sets the amount to change the volume after processing (in decibels).
  // This can be used to compensate for the decrease in volume resulting from compression. This is only applied to the wet signal, not the dry signal.
  public native func SetMakeupGain(makeupGain: Float, opt tween: ref<Tween>);

  // Sets how much dry (unprocessed) signal should be blended with the wet (processed) signal.
  // Valid mix values range from 0.0 to 1.0, where 0.0 is the dry signal only, 1.0 is the wet signal only, and 0.5 is an equal mix of both.
  public native func SetMix(mix: Float, opt tween: ref<Tween>);

  public native static func Create(
    threshold: Float,
    ratio: Float,
    attackDuration: Float,
    releaseDuration: Float,
    makeupGain: Float,
    mix: Float
  ) -> ref<DynamicCompressor>;
}

// The frequencies that the filter will remove.
// see: https://docs.rs/kira/latest/kira/effect/filter/enum.FilterMode.html
enum FilterMode {
  LowPass = 0,
  BandPass = 1,
  HighPass = 2,
  Notch = 3,
}

public native class DynamicFilter extends DynamicEffect {
  // Sets the frequencies that the filter will remove.
  public native func SetMode(mode: FilterMode);

  // Sets the cutoff frequency of the filter (in hertz).
  public native func SetCutoff(cutoff: Float, opt tween: ref<Tween>);

  // Sets the resonance of the filter.
  public native func SetResonance(resonance: Float, opt tween: ref<Tween>);

  // Sets how much dry (unprocessed) signal should be blended with the wet (processed) signal.
  // Valid mix values range from 0.0 to 1.0, where 0.0 is the dry signal only, 1.0 is the wet signal only, and 0.5 is an equal mix of both.
  public native func SetMix(mix: Float, opt tween: ref<Tween>);

  public native static func Create(mode: FilterMode, cutoff: Float, resonance: Float, mix: Float) -> ref<DynamicFilter>;
}

public native class DynamicReverb extends DynamicEffect {
  // Sets how much the room reverberates. A higher value will result in a bigger sounding room. 1.0 gives an infinitely reverberating room.
  public native func SetFeedback(feedback: Float, opt tween: ref<Tween>);

  // Sets how quickly high frequencies disappear from the reverberation.
  public native func SetDamping(damping: Float, opt tween: ref<Tween>);

  // Sets the stereo width of the reverb effect (0.0 being fully mono, 1.0 being fully stereo).
  public native func SetStereoWidth(stereoWidth: Float, opt tween: ref<Tween>);

  // Sets how much dry (unprocessed) signal should be blended with the wet (processed) signal.
  // Valid mix values range from 0.0 to 1.0, where 0.0 is the dry signal only, 1.0 is the wet signal only, and 0.5 is an equal mix of both.
  public native func SetMix(mix: Float, opt tween: ref<Tween>);

  public native static func Create(feedback: Float, damping: Float, stereoWidth: Float, mix: Float) -> ref<DynamicFilter>;
}

