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
    let emitterID: EntityID;
    let emitterName: CName;

    let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
    emitterID = target.GetEntityID();
    emitterName = n"Jean-Guy";
    let added = GameInstance.GetAudioSystemExt(game).RegisterEmitter(emitterID, emitterName);
    FTLog(s"registered? \(added)");
}
/// Game.TestUnregisterEmitter();
public static exec func TestUnregisterEmitter(game: GameInstance) {
    let emitterID: EntityID;
    let emitterName: CName;

    let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
    emitterID = target.GetEntityID();
    emitterName = n"Jean-Guy";
    let added = GameInstance.GetAudioSystemExt(game).UnregisterEmitter(emitterID);
    FTLog(s"unregistered? \(added)");
}

/// Game.TestPlayOnEmitter("straight_outta_compton", "Eazy-E");
public static exec func TestPlayOnEmitter(game: GameInstance, soundName: String, opt emitterName: String) {
    let soundCName = StringToName(soundName);
    let emitterID: EntityID;
    let emitterCName: CName = IsNameValid(StringToName(emitterName))
    ? StringToName(emitterName)
    : n"Unknown name";

    let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
    emitterID = target.GetEntityID();
    GameInstance.GetAudioSystemExt(game).RegisterEmitter(emitterID);
    GameInstance.GetAudioSystemExt(game).PlayOnEmitter(soundCName, emitterID, emitterCName);
}