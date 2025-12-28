module Audioware

public native class AudioEventCallbackSystem {
    public native final func RegisterCallback(eventName: CName, target: ref<IScriptable>, functionName: CName) -> ref<AudioEventCallbackHandler>;
    public native final func RegisterStaticCallback(eventName: CName, className: CName, functionName: CName) -> ref<AudioEventCallbackHandler>;
}

public abstract native class SoundEvent {}

public abstract native class WwiseEvent extends SoundEvent {
    public final native func WwiseID() -> Uint32;
}

public abstract native class EmitterEvent extends WwiseEvent {
    public final native func EntityID() -> EntityID;
    public final native func EmitterName() -> CName;
}

//public abstract native class MetadataEvent extends EmitterEvent {
//    public final native func MetadataName() -> CName;
//}

public native class PlayEvent extends EmitterEvent {
    public final native func EventName() -> CName;
    public final native func SoundTags() -> array<CName>;
    public final native func EmitterTags() -> array<CName>;
    public final native func Seek() -> Float;
}

public native class PlayExternalEvent extends PlayEvent {
    public final native func ExternalResourcePath() -> Uint64;
}

public native class PlayOneShotEvent extends PlayEvent {
    public final native func Params() -> array<AudParam>;
    public final native func Switches() -> array<audioAudSwitch>;
}

public native class StopSoundEvent extends EmitterEvent {
    public final native func EventName() -> CName;
    public final native func FloatData() -> Float;
}

public native class StopTaggedEvent extends EmitterEvent {
    public final native func TagName() -> CName;
}

public native class SetParameterEvent extends EmitterEvent {
    public final native func NameData() -> CName;
    public final native func FloatData() -> Float;
}

public native class SetGlobalParameterEvent extends WwiseEvent {
    public final native func Name() -> CName;
    public final native func Value() -> Float;
    public final native func Duration() -> Float;
    public final native func CurveType() -> audioESoundCurveType;
}

public native class SetSwitchEvent extends SoundEvent {
    public final native func SwitchName() -> CName;
    public final native func SwitchValue() -> CName;
    public final native func SwitchNameWwiseID() -> Uint32;
    public final native func SwitchValueWwiseID() -> Uint32;
}

public native class SetAppearanceNameEvent extends EmitterEvent {
    public final native func Name() -> CName;
}

public native class SetEntityNameEvent extends EmitterEvent {
    public final native func Name() -> CName;
}

public native class TagEvent extends EmitterEvent {
    public final native func TagName() -> CName;
}

public native class UntagEvent extends EmitterEvent {
    public final native func TagName() -> CName;
}

public native class AddContainerStreamingPrefetchEvent extends EmitterEvent {
    public final native func EventName() -> CName;
}

public native class RemoveContainerStreamingPrefetchEvent extends EmitterEvent {
    public final native func EventName() -> CName;
}

public struct AudParam {
    public let name: CName;
    public let value: Float;
}

public native class AudioEventCallbackHandler {
    public final native func Unregister();
    public final native func IsRegistered() -> Bool;
    public final native func AddTarget(value: ref<AudioEventCallbackTarget>) -> ref<AudioEventCallbackHandler>;
    public final native func RemoveTarget(value: ref<AudioEventCallbackTarget>) -> ref<AudioEventCallbackHandler>;
    public final native func SetLifetime(lifetime: AudioEventCallbackLifetime) -> ref<AudioEventCallbackHandler>;
}

public abstract native class AudioEventCallbackTarget {}
public native class EntityTarget extends AudioEventCallbackTarget {
    public static native func EntityID(entityID: EntityID) -> ref<EntityTarget>;
    public static native func EmitterName(emitterName: CName) -> ref<EntityTarget>;
}
public native class EventTarget extends AudioEventCallbackTarget {
    public static native func ActionType(eventType: audioEventActionType) -> ref<EventTarget>;
    public static native func ActionTypeName(eventType: String) -> ref<EventTarget>;
}

public enum AudioEventCallbackLifetime {
    Session = 0,
    Forever = 1,
}
