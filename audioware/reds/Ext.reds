import Audioware.AudiowareSystem
import Audioware.EmitterSettings
import Audioware.Tween
import Audioware.AudioSettingsExt
import Audioware.AudioSettingsExtBuilder

@addMethod(GameInstance)
public static func GetAudioSystemExt(game: GameInstance) -> ref<AudioSystemExt> {
    return new AudioSystemExt();
}

public native class AudioSystemExt {
    // enhanced SDK
    public final native func Play(eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt line: scnDialogLineType, opt ext: ref<AudioSettingsExt>) -> Void;
    public final func Play(eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt line: scnDialogLineType, opt tween: ref<Tween>) -> Void {
        let builder = AudioSettingsExtBuilder.Create();
        builder.SetFadeInTween(tween);
        let settings = builder.Build();
        this.Play(eventName, entityID, emitterName, line, settings);
    }
    public native func Stop(eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt tween: ref<Tween>) -> Void
    public native func Switch(switchName: CName, switchValue: CName, opt entityID: EntityID, opt emitterName: CName, opt switchNameTween: ref<Tween>, opt switchValueTween: ref<Tween>) -> Void;
    public native func PlayOverThePhone(eventName: CName, emitterName: CName, gender: CName) -> Void;
        
    // spatial scene
    public func RegisterEmitter(entityID: EntityID, opt emitterName: CName, opt emitterSettings: EmitterSettings) -> Void { AudiowareSystem.GetInstance(GetGameInstance()).RegisterEmitter(entityID, emitterName, emitterSettings); }
    public func UnregisterEmitter(entityID: EntityID) -> Void { AudiowareSystem.GetInstance(GetGameInstance()).UnregisterEmitter(entityID); }
    public func IsValidEmitter(className: CName) -> Bool { return AudiowareSystem.GetInstance(GetGameInstance()).IsValidEmitter(className); }
    public native func IsRegisteredEmitter(entityID: EntityID) -> Bool;
    public native func EmittersCount() -> Int32;
    public native func PlayOnEmitter(eventName: CName, entityID: EntityID, emitterName: CName, opt tween: ref<Tween>) -> Void;
    public native func StopOnEmitter(eventName: CName, entityID: EntityID, emitterName: CName, opt tween: ref<Tween>) -> Void;
    public native func OnEmitterDies(entityID: EntityID) -> Void;
    public native func OnEmitterDying(entityID: EntityID) -> Void;
}
