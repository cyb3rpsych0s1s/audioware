module Audioware

public native class AudioEventManager {
  public final native func Mute(eventName: CName);
  public final native func Unmute(eventName: CName);
  public final native func IsMuted(eventName: CName);
  public final func MuteSpecific(eventName: CName, eventType: audioEventActionType)     { this.MuteSpecific(eventName, ToBits(eventType)); }
  public final func MuteSpecific(eventName: CName, eventType: [audioEventActionType])   { this.MuteSpecific(eventName, ToBits(eventType)); }
  public final func MuteSpecific(eventName: CName, eventType: EventHookType)            { this.MuteSpecific(eventName, ToBits(eventType)); }
  public final func MuteSpecific(eventName: CName, eventType: [EventHookType])          { this.MuteSpecific(eventName, ToBits(eventType)); }
  public final func UnmuteSpecific(eventName: CName, eventType: audioEventActionType)   { this.MuteSpecific(eventName, ToBits(eventType)); }
  public final func UnmuteSpecific(eventName: CName, eventType: [audioEventActionType]) { this.MuteSpecific(eventName, ToBits(eventType)); }
  public final func UnmuteSpecific(eventName: CName, eventType: EventHookType)          { this.MuteSpecific(eventName, ToBits(eventType)); }
  public final func UnmuteSpecific(eventName: CName, eventType: [EventHookType])        { this.MuteSpecific(eventName, ToBits(eventType)); }
  public final func IsSpecificMuted(eventName: CName, eventType: audioEventActionType) -> Bool = this.IsSpecificMuted(eventName, ToBits(eventType));
  public final func IsSpecificMuted(eventName: CName, eventType: EventHookType) -> Bool = this.IsSpecificMuted(eventName, ToBits(eventType));
  public final native func IsSpecificMuted(eventName: CName, eventTypeBits: Uint32) -> Bool;
  public final native func MuteSpecific(eventName: CName, eventTypeBits: Uint32);
  public final native func UnmuteSpecific(eventName: CName, eventTypeBits: Uint32);
}

@addMethod(GameInstance)
public static final func GetAudioEventManager() -> ref<AudioEventManager> = new AudioEventManager();

private func ToBits(variant: audioEventActionType) -> Uint32 = BitShiftL32(1u, EnumInt(variant) + 1);
private func ToBits(variants: [audioEventActionType]) -> Uint32 {
    let bits = 0u;
    for variant in variants {
        BitSet32(bits, EnumInt(variant) + 1, true); 
    }
    return bits;
}

private func ToBits(variant: EventHookType) -> Uint32 = BitShiftL32(1u, EnumInt(variant));
private func ToBits(variants: [EventHookType]) -> Uint32 {
    let bits = 0u;
    for variant in variants {
        BitSet32(bits, EnumInt(variant), true); 
    }
    return bits;
}
