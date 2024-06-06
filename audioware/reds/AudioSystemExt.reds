module Audioware

@addMethod(AudioSystem)
public func Stop(eventName: CName, opt entityID: EntityID, opt emitterName: CName, tween: ref<AudiowareTween>) -> Void {
    AudiowareTrackStop(eventName, entityID, emitterName, tween);
}
