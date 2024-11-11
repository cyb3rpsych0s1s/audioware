import Audioware.AudioSettingsExt
import Audioware.Yolo

/// Game.TestPlayExt("straight_outta_compton");
public static exec func TestPlayExt(game: GameInstance, name: String) {
    // GameInstance.GetAudioSystemExt(game).Play(StringToName(name));
    let settings: ref<AudioSettingsExt> = new AudioSettingsExt();
    settings.startPosition = 1.3;
    Yolo(settings);
}