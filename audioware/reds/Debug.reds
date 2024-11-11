import Audioware.AudioSettingsExt
import Audioware.Tween

/// Game.TestPlayExt("straight_outta_compton");
public static exec func TestPlayExt(game: GameInstance, name: String) {
    GameInstance.GetAudioSystemExt(game).Play(StringToName(name));
}
/// Game.TestStop("straight_outta_compton");
public static exec func TestStop(game: GameInstance, name: String) {
    GameInstance.GetAudioSystemExt(game).Stop(StringToName(name));
}