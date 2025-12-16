module Audioware

public native class AudioEventManager {
  public final native func Mute(eventName: CName);
  public final native func MuteSpecific(eventName: CName, eventType: audioEventActionType);
  public final native func IsMuted(eventName: CName);
  public final native func IsSpecificMuted(eventName: CName, eventType: audioEventActionType);
}

