module Audioware

@addMethod(AudioSystem)
public func Stop(eventName: CName, opt entityID: EntityID, opt emitterName: CName, tween: LinearTween) -> Void {
    StopLinear(eventName, entityID, emitterName, tween);
}
@addMethod(AudioSystem)
public func Stop(eventName: CName, opt entityID: EntityID, opt emitterName: CName, tween: ElasticTween) -> Void {
    StopElastic(eventName, entityID, emitterName, tween);
}