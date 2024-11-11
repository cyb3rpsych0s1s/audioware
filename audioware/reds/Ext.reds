import Audioware.AudioSettingsExt

// @addMethod(GameInstance)
// public static func GetAudioSystemExt(game: GameInstance) -> ref<AudioSystemExt> = new AudioSystemExt();

public native class AudioSystemExt {
    // enhanced SDK
    public final native func Play(eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt line: scnDialogLineType, opt ext: ref<AudioSettingsExt>) -> Void;
}