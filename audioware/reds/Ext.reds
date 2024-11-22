import Audioware.AudioSettingsExt
import Audioware.Tween
import Audioware.EmitterDistances
import Audioware.EmitterSettings

@addMethod(GameInstance)
public static func GetAudioSystemExt(game: GameInstance) -> ref<AudioSystemExt> = new AudioSystemExt();

public native class AudioSystemExt {
    // enhanced SDK
    public final native func Play(eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt line: scnDialogLineType, opt ext: ref<AudioSettingsExt>) -> Void;
    public final native func Stop(eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt tween: ref<Tween>) -> Void;
    public final native func Switch(switchName: CName, switchValue: CName, opt entityID: EntityID, opt emitterName: CName, opt switchNameTween: ref<Tween>, opt switchValueExt: ref<AudioSettingsExt>) -> Void;
    // enhanced SDK variants
    public final func Play(eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt line: scnDialogLineType, opt tween: ref<Tween>) -> Void {
        let ext = new AudioSettingsExt();
        ext.fadeIn = tween;
        this.Play(eventName, entityID, emitterName, line, ext);
    }
    public final func Switch(switchName: CName, switchValue: CName, opt entityID: EntityID, opt emitterName: CName, opt switchNameTween: ref<Tween>, opt switchValueTween: ref<Tween>) -> Void {
        let ext = new AudioSettingsExt();
        ext.fadeIn = switchValueTween;
        this.Switch(switchName, switchValue, entityID, emitterName, switchNameTween, ext); 
    }

    // spatial scene
    public final native func IsRegisteredEmitter(entityID: EntityID) -> Bool;
    public final native func RegisterEmitter(entityID: EntityID, opt emitterName: CName, opt emitterSettings: ref<EmitterSettings>) -> Bool;
    public final native func UnregisterEmitter(entityID: EntityID) -> Bool;
    public final native func PlayOnEmitter(eventName: CName, entityID: EntityID, emitterName: CName, opt ext: ref<AudioSettingsExt>) -> Void;
    public final native func StopOnEmitter(eventName: CName, entityID: EntityID, emitterName: CName, opt tween: ref<Tween>) -> Void;
    public final native func OnEmitterDies(entityID: EntityID) -> Void;
    public final native func OnEmitterIncapacitated(entityID: EntityID) -> Void;
    public final native func OnEmitterDefeated(entityID: EntityID) -> Void;
}
