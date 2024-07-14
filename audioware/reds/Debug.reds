import Audioware.LocalizationPackage
import Codeware.Localization.*

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
    package.DefineSubtitles();
    let text = LocalizationSystem.GetInstance(game).GetSubtitle("custom_subtitle");
    FTLog(AsRef(text));
}

/// Game.TestAudioSystemPlay("ono_v_effort_short");
/// Game.TestAudioSystemPlay("nah_everything_is_all_good");
/// Game.TestAudioSystemPlay("as_if_I_didnt_know_already");
public static exec func TestAudioSystemPlay(game: GameInstance, name: String) {
    let cname = StringToName(name);
    GameInstance.GetAudioSystem(game).Play(cname);
}
