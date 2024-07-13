import Audioware.LocalizationPackage
import Audioware.DefineSubtitles

/// Game.TestRegisterEmitter()
public static exec func TestRegisterEmitter(game: GameInstance) {
    let emitterID: EntityID;
    let emitterName: CName;

    let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
    emitterID = target.GetEntityID();
    emitterName = n"Jean-Guy";

    let audioSystem = GameInstance.GetAudioSystem(game);
    audioSystem.RegisterEmitter(emitterID, emitterName);
}

/// Game.TestUnregisterEmitter()
public static exec func TestUnregisterEmitter(game: GameInstance) {
    let emitterID: EntityID;

    let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
    emitterID = target.GetEntityID();

    let audioSystem = GameInstance.GetAudioSystem(game);
    audioSystem.UnregisterEmitter(emitterID);
}

/// Game.TestDefineSubtitles();
public static exec func TestDefineSubtitles(game: GameInstance) {
    let package = new LocalizationPackage();
    DefineSubtitles(package);
}
