module Audioware

// TODO: refactor when interfaces land in Redscript.

public native class AudioEventCallbackSystem {
    public native final func RegisterCallback(eventName: CName, target: ref<IScriptable>, functionName: CName) -> ref<AudioEventCallbackHandler>;
    public native final func RegisterStaticCallback(eventName: CName, className: CName, functionName: CName) -> ref<AudioEventCallbackHandler>;
}

public abstract native class SoundEvent {
    public func PrimaryName() -> CName;
    public final static func InvalidWwiseID() -> Uint32 = 2166136261u;
}

public native class PlayEvent extends SoundEvent {
    public final native func EventName() -> CName;
    
    public final native func EntityID() -> EntityID;
    public final native func EmitterName() -> CName;
    public final native func Position() -> Vector4;
    public final native func WwiseID() -> Uint32;
    
    public final native func SoundTags() -> array<CName>;
    public final native func EmitterTags() -> array<CName>;
    public final native func Seek() -> Float;
    
    public func PrimaryName() -> CName = this.EventName();
}

public native class PlayExternalEvent extends SoundEvent {
    public final native func EventName() -> CName;
    
    public final native func EntityID() -> EntityID;
    public final native func EmitterName() -> CName;
    public final native func Position() -> Vector4;
    public final native func WwiseID() -> Uint32;
    
    public final native func SoundTags() -> array<CName>;
    public final native func EmitterTags() -> array<CName>;
    public final native func Seek() -> Float;
    
    public final native func ExternalResourcePath() -> ResRef;
    
    public func PrimaryName() -> CName = this.EventName();
}

public native class PlayOneShotEvent extends SoundEvent {
    public final native func EventName() -> CName;
    
    public final native func EntityID() -> EntityID;
    public final native func EmitterName() -> CName;
    public final native func Position() -> Vector4;
    public final native func WwiseID() -> Uint32;
    
    public final native func Params() -> array<AudParam>;
    public final native func Switches() -> array<audioAudSwitch>;
    public final native func GraphOcclusion() -> Float;
    public final native func RaycastOcclusion() -> Float;
    public final native func HasGraphOcclusion() -> Bool;
    public final native func HasRaycastOcclusion() -> Bool;
    public final native func IsInDifferentRoom() -> Bool;
    
    public func PrimaryName() -> CName = this.EventName();
}

public native class StopSoundEvent extends SoundEvent {
    public final native func SoundName() -> CName;
    
    public final native func EntityID() -> EntityID;
    public final native func WwiseID() -> Uint32;
    
    public final native func FadeOut() -> Float;
    
    public func PrimaryName() -> CName = this.SoundName();
}

public native class StopTaggedEvent extends SoundEvent {
    public final native func TagName() -> CName;
    
    public final native func EntityID() -> EntityID;
    public final native func WwiseID() -> Uint32;
    
    public func PrimaryName() -> CName = this.TagName();
}

public native class SetParameterEvent extends SoundEvent {
    public final native func ParamName() -> CName;
    public final native func ParamValue() -> Float;
    
    public final native func EntityID() -> EntityID;
    public final native func EmitterName() -> CName;
    public final native func Position() -> Vector4;
    public final native func WwiseID() -> Uint32;
    
    public final native func SoundTags() -> array<CName>;
    public final native func EmitterTags() -> array<CName>;
    
    public func PrimaryName() -> CName = this.ParamName();
}

public native class SetGlobalParameterEvent extends SoundEvent {
    public final native func Name() -> CName;
    public final native func Value() -> Float;
    public final native func Duration() -> Float;
    public final native func CurveType() -> audioESoundCurveType;
    
    public final native func WwiseID() -> Uint32;
    
    public func PrimaryName() -> CName = this.Name();
}

public native class SetSwitchEvent extends SoundEvent {
    public final native func SwitchName() -> CName;
    public final native func SwitchValue() -> CName;
    
    public final native func SwitchNameWwiseID() -> Uint32;
    public final native func SwitchValueWwiseID() -> Uint32;
    
    public final native func EntityID() -> EntityID;
    public final native func EmitterName() -> CName;
    public final native func Position() -> Vector4;
    
    public final native func SoundTags() -> array<CName>;
    public final native func EmitterTags() -> array<CName>;
    
    public func PrimaryName() -> CName = this.SwitchName();
}

public native class SetAppearanceNameEvent extends SoundEvent {
    public final native func Name() -> CName;
    
    public final native func EntityID() -> EntityID;
    public final native func WwiseID() -> Uint32;
    
    public func PrimaryName() -> CName = this.Name();
}

public native class SetEntityNameEvent extends SoundEvent {
    public final native func Name() -> CName;
    
    public final native func EntityID() -> EntityID;
    public final native func WwiseID() -> Uint32;
    
    public func PrimaryName() -> CName = this.Name();
}

public native class TagEvent extends SoundEvent {
    public final native func TagName() -> CName;
    
    public final native func EntityID() -> EntityID;
    public final native func WwiseID() -> Uint32;
    
    public func PrimaryName() -> CName = this.TagName();
}

public native class UntagEvent extends SoundEvent {
    public final native func TagName() -> CName;
    
    public final native func EntityID() -> EntityID;
    public final native func WwiseID() -> Uint32;
    
    public func PrimaryName() -> CName = this.TagName();
}

public native class AddContainerStreamingPrefetchEvent extends SoundEvent {
    public final native func EventName() -> CName;
    
    public final native func EntityID() -> EntityID;
    public final native func WwiseID() -> Uint32;
    
    public func PrimaryName() -> CName = this.EventName();
}

public native class RemoveContainerStreamingPrefetchEvent extends SoundEvent {
    public final native func EventName() -> CName;
    
    public final native func EntityID() -> EntityID;
    public final native func WwiseID() -> Uint32;
    
    public func PrimaryName() -> CName = this.EventName();
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
