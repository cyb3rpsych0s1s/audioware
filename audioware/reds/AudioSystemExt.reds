module Audioware

private native func PlayOverThePhone(eventName: CName, emitterName: CName) -> Void;

@addMethod(AudioSystem)
public func PlayOverThePhone(eventName: CName, emitterName: CName) -> Void {
    PlayOverThePhone(eventName, emitterName);
}
