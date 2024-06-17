module Audioware

@addMethod(AudioSystem)
public func PlayOnTrack(eventName: CName, trackName: CName, opt entityID: EntityID, opt emitterName: CName, opt tween: ref<AudiowareTween>) -> Void {
    AudiowarePlayOnTrack(eventName, trackName, entityID, emitterName, tween);
}

@addMethod(AudioSystem)
public func Stop(eventName: CName, opt entityID: EntityID, opt emitterName: CName, tween: ref<AudiowareTween>) -> Void {
    AudiowareTrackStop(eventName, entityID, emitterName, tween);
}

@addMethod(AudioSystem)
public func AddTrack(trackName: CName) -> Void {
    AudiowareAddTrack(trackName);
}

@addMethod(AudioSystem)
public func RemoveTrack(trackName: CName) -> Void {
    AudiowareRemoveTrack(trackName);
}
