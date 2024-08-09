import Audioware.AudiowareSystem
import Audioware.EmitterSettings
import Audioware.Tween
import Audioware.ArgsExt

@addMethod(GameInstance)
public static func GetAudioSystemExt(game: GameInstance) -> AudioSystemExt {
    return new AudioSystemExt();
}

public native struct AudioSystemExt {
    // enhanced SDK
    public static native func Play(self: AudioSystemExt, eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt line: scnDialogLineType, opt tween: ref<Tween>) -> Void;
    public static native func PlayWith(self: AudioSystemExt, eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt line: scnDialogLineType, opt ext: ref<ArgsExt>) -> Void;
    public static native func Stop(self: AudioSystemExt, eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt tween: ref<Tween>) -> Void
    public static native func Switch(self: AudioSystemExt, switchName: CName, switchValue: CName, opt entityID: EntityID, opt emitterName: CName, opt switchNameTween: ref<Tween>, opt switchValueTween: ref<Tween>) -> Void;
    public static native func PlayOverThePhone(self: AudioSystemExt, eventName: CName, emitterName: CName, gender: CName) -> Void;
    
    // spatial scene
    public static func RegisterEmitter(self: AudioSystemExt, entityID: EntityID, opt emitterName: CName, opt emitterSettings: EmitterSettings) -> Void { AudiowareSystem.GetInstance(GetGameInstance()).RegisterEmitter(entityID, emitterName, emitterSettings); }
    public static func UnregisterEmitter(self: AudioSystemExt, entityID: EntityID) -> Void { AudiowareSystem.GetInstance(GetGameInstance()).UnregisterEmitter(entityID); }
    public static func IsValidEmitter(self: AudioSystemExt, className: CName) -> Bool { return AudiowareSystem.GetInstance(GetGameInstance()).IsValidEmitter(className); }
    public static native func IsRegisteredEmitter(self: AudioSystemExt, entityID: EntityID) -> Bool;
    public static native func EmittersCount(self: AudioSystemExt) -> Int32;
    public static native func PlayOnEmitter(self: AudioSystemExt, eventName: CName, entityID: EntityID, emitterName: CName, opt tween: ref<Tween>) -> Void;
    public static native func StopOnEmitter(self: AudioSystemExt, eventName: CName, entityID: EntityID, emitterName: CName, opt tween: ref<Tween>) -> Void;
}
