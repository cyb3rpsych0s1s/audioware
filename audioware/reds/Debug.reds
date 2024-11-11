import Audioware.AudioSettingsExt
import Audioware.Tween
import Audioware.EmitterDistances
import Audioware.EmitterSettings

/// Game.TestPlayExt("straight_outta_compton");
public static exec func TestPlayExt(game: GameInstance, name: String) {
    GameInstance.GetAudioSystemExt(game).Play(StringToName(name));
}
/// Game.TestStop("straight_outta_compton");
public static exec func TestStop(game: GameInstance, name: String) {
    GameInstance.GetAudioSystemExt(game).Stop(StringToName(name));
}
/// Game.TestRegisterEmitter();
public static exec func TestRegisterEmitter(game: GameInstance) {
    let v = GetPlayer(game).GetEntityID();
    let added = GameInstance.GetAudioSystemExt(game).RegisterEmitter(v);
    FTLog(s"registered? \(added)");
}
/// Game.TestUnregisterEmitter();
public static exec func TestUnregisterEmitter(game: GameInstance) {
    let v = GetPlayer(game).GetEntityID();
    let added = GameInstance.GetAudioSystemExt(game).UnregisterEmitter(v);
    FTLog(s"unregistered? \(added)");
}