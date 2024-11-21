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
