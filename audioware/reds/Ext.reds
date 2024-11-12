import Audioware.AudioSettingsExt
import Audioware.Tween
import Audioware.EmitterDistances
import Audioware.EmitterSettings
import Audioware.RegisterEmitter
import Audioware.UnregisterEmitter

@addMethod(GameInstance)
public static func GetAudioSystemExt(game: GameInstance) -> ref<AudioSystemExt> = new AudioSystemExt();

public native class AudioSystemExt {
    // enhanced SDK
    public final native func Play(eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt line: scnDialogLineType, opt ext: ref<AudioSettingsExt>) -> Void;
    public final native func Stop(eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt tween: ref<Tween>) -> Void;

    // spatial scene
    public final func RegisterEmitter(entityID: EntityID, opt emitterName: CName, opt emitterSettings: EmitterSettings) -> Bool = RegisterEmitter(entityID, emitterName, emitterSettings);
    public final func RegisterEmitter(entityID: EntityID, opt emitterName: CName, opt emitterSettings: EmitterSettings) -> Void { let _ = RegisterEmitter(entityID, emitterName, emitterSettings); } // avoid crash on return type
    public final func UnregisterEmitter(entityID: EntityID) -> Bool = UnregisterEmitter(entityID);
    public final func UnregisterEmitter(entityID: EntityID) -> Void { let unregistered = UnregisterEmitter(entityID); } // avoid crash on return type
    public final native func PlayOnEmitter(eventName: CName, entityID: EntityID, emitterName: CName, opt tween: ref<Tween>) -> Void;
}
