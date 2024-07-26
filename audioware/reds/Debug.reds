import Audioware.LocalizationPackage
import Audioware.TestPlay
import Audioware.StopOnEmitter
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

/// Game.TestAudioSystemStop("god_love_us");
/// Game.TestAudioSystemStop("coco_caline");
/// Game.TestAudioSystemStop("copacabana");
public static exec func TestAudioSystemStop(game: GameInstance, name: String) {
    let cname = StringToName(name);
    GameInstance.GetAudioSystem(game).Stop(cname);
}

/// Game.TestAudioSystemSwitch("coco_caline", "copacabana");
public static exec func TestAudioSystemSwitch(game: GameInstance, prev: String, next: String) {
    let name = StringToName(prev);
    let value = StringToName(next);
    GameInstance.GetAudioSystem(game).Switch(name, value);
}

/// Game.TestAudioSystemPlayOnEmitter("ono_v_effort_short");
/// Game.TestAudioSystemPlayOnEmitter("nah_everything_is_all_good");
/// Game.TestAudioSystemPlayOnEmitter("as_if_I_didnt_know_already");
/// Game.TestAudioSystemPlayOnEmitter("god_love_us");
/// Game.TestAudioSystemPlayOnEmitter("coco_caline");
/// Game.TestAudioSystemPlayOnEmitter("copacabana");
public static exec func TestAudioSystemPlayOnEmitter(game: GameInstance, name: String) {
    let cname = StringToName(name);
    let emitterID: EntityID;

    let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
    emitterID = target.GetEntityID();

    GameInstance.GetAudioSystem(game).PlayOnEmitter(cname, emitterID, n"Jean-Michel");
}

/// Game.TestAudioSystemStopOnEmitter("coco_caline");
public static exec func TestAudioSystemStopOnEmitter(game: GameInstance, name: String) {
    let cname = StringToName(name);
    let emitterID: EntityID;

    let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
    emitterID = target.GetEntityID();

    StopOnEmitter(cname, emitterID, n"Jean-Michel");
}

/// Game.TestAudioSystemParameter("Audioware:Reverb", 1.0);
/// Game.TestAudioSystemParameter("Audioware:Reverb", 0.0);
public static exec func TestAudioSystemParameter(game: GameInstance, name: String, value: Float) {
    let cname = StringToName(name);
    GameInstance.GetAudioSystem(game).Parameter(cname, value);
}

/// Game.TestPlayRustOnly();
public static exec func TestPlayRustOnly(game: GameInstance) {
    TestPlay();
}

/// Game.TestScenePositions();
public static exec func TestScenePositions(game: GameInstance) {
    let player = GetPlayer(game);
    let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(player);
    LogPositions(player, "listener");
    LogPositions(target, "emitter");
}

private func LogVector4(position: Vector4, name: String) {
    let x = position.X;
    let y = position.Y;
    let z = position.Z;
    FTLog(AsRef(s"\(name) => x: \(ToString(x)), y: \(ToString(y)), z: \(ToString(z))"));
}

private func LogQuaternion(orientation: Quaternion, name: String) {
    let i = orientation.i;
    let j = orientation.j;
    let k = orientation.k;
    let r = orientation.r;
    FTLog(AsRef(s"\(name) => i: \(ToString(i)), j: \(ToString(j)), k: \(ToString(k)), r: \(ToString(r))"));
}

private func LogPositions(entity: ref<Entity>, name: String) {
    let position = entity.GetWorldPosition();
    let forward = entity.GetWorldForward();
    let up = entity.GetWorldUp();
    let right = entity.GetWorldRight();
    let orientation = entity.GetWorldOrientation();
    FTLog(AsRef(s"== \(name) =="));
    LogVector4(position, "world position");
    LogVector4(forward, "world forward");
    LogVector4(up, "world up");
    LogVector4(right, "world right");
    LogQuaternion(orientation, "world orientation");
    FTLog(AsRef(s"============"));
}
