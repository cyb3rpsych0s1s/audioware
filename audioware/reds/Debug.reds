import Codeware.Localization.*
import Audioware.*

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

/// Game.TestAudioSystemPlay("dimanche_aux_goudes");
/// Game.TestAudioSystemPlay("dok_mai_gab_jeh_gan");
/// Game.TestAudioSystemPlay("ton");
public static exec func TestAudioSystemPlay(game: GameInstance, name: String) {
    let cname = StringToName(name);
    GameInstance.GetAudioSystem(game).Play(cname);
}

/// Game.TestAudioSystemStopSmoothly("dimanche_aux_goudes");
public static exec func TestAudioSystemStopSmoothly(game: GameInstance, name: String) {
    let cname = StringToName(name);
    let tween = AudiowareLinearTween.Immediate(5.);
    let nope: EntityID;
    let none: CName;
    GameInstance.GetAudioSystem(game).Stop(cname, nope, none, tween);
}

/// Game.TestAudioSystemPlayOnV("ono_v_effort_short");
/// Game.TestAudioSystemPlayOnV("nah_everything_is_all_good");
/// Game.TestAudioSystemPlayOnV("as_if_I_didnt_know_already");
public static exec func TestAudioSystemPlayOnV(game: GameInstance, name: String) {
    let cname = StringToName(name);
    let player = GetPlayer(game);
    GameInstance.GetAudioSystem(game).Play(cname, player.GetEntityID(), n"V");
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
    if !GameInstance.GetAudioSystem(game).IsRegisteredEmitter(emitterID) {
        GameInstance.GetAudioSystem(game).RegisterEmitter(emitterID);
    }

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

/// Game.TestAudioSystemPlayOverThePhone("nah_everything_is_all_good");
/// Game.TestAudioSystemPlayOverThePhone("as_if_I_didnt_know_already");
public static exec func TestAudioSystemPlayOverThePhone(game: GameInstance, name: String) {
    let cname = StringToName(name);
    GameInstance.GetAudioSystem(game).PlayOverThePhone(cname, n"Vik", n"Male");
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

/// Game.TestOtis();
public static exec func TestOtis(game: GameInstance) {
    GameInstance.GetAudioSystem(game).Play(n"situation_scribe", GetPlayer(game).GetEntityID(), n"V");
    
    let callback = new OtisReplyCallback();
    callback.player = GetPlayer(game);
    GameInstance
        .GetDelaySystem(GetGameInstance())
        .DelayCallback(callback, 3.0);
}

public class OtisReplyCallback extends DelayCallback {
    public let player: wref<PlayerPuppet>;
    public func Call() -> Void {
        if !IsDefined(this.player) { return; }
        let game = this.player.GetGame();
        let emitterID: EntityID;
        let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(this.player);
        emitterID = target.GetEntityID();
        if !GameInstance.GetAudioSystem(game).IsRegisteredEmitter(emitterID) {
            GameInstance.GetAudioSystem(game).RegisterEmitter(emitterID);
        }
        GameInstance.GetAudioSystem(game).PlayOnEmitter(n"monologue_otis", emitterID, n"Otis");
    }
}

/// Game.TestAmbience();
public static exec func TestAmbience(game: GameInstance) {
    let weather = GameInstance.GetWeatherSystem(game);
    weather.SetWeather(n"24h_weather_rain", 20.0, 9u);
    GameInstance.GetAudioSystem(game).Play(n"milles_feuilles");
}

/// Game.TestAudioSystemPlayOnV("feature_parameter_intro");
/// Game.TestReverb(1.0);
/// Game.TestAudioSystemPlayOnV("feature_parameter_reverb");
/// Game.TestAudioSystemPlayOnV("feature_parameter_noreverb");
/// Game.TestAudioSystemPlay("feel_good_inc");
/// Game.TestAudioSystemStopSmoothly("feel_good_inc");
/// Game.TestReverb(0.0);
/// Game.TestAudioSystemPlayOnV("feature_parameter_preset");
/// Game.TestPreset("Underwater");
/// Game.TestAudioSystemPlayOnV("feature_parameter_underwater");
/// Game.TestPreset("OnThePhone");
/// Game.TestAudioSystemPlayOnV("feature_parameter_onthephone");
/// Game.TestPreset("None");
/// Game.TestAudioSystemPlayOnV("feature_parameter_outro");

/// Game.TestAudioSystemPlayOnV("feature_fail");
/// Game.TestPreset("Underwater");
/// Game.TestAudioSystemPlayOnV("feature_repeat");
/// Game.TestPreset("OnThePhone");
/// Game.TestAudioSystemPlayOnV("feature_repeat");
/// Game.TestPreset("None");
/// Game.TestAudioSystemPlayOnV("feature_repeat");

/// Game.TestReverb(1.0);
/// Game.TestReverb(0.0);
public static exec func TestReverb(game: GameInstance, reverb: Float) {
    GameInstance.GetBlackboardSystem(game)
    .Get(GetAllBlackboardDefs().Audioware_Settings)
    .SetFloat(GetAllBlackboardDefs().Audioware_Settings.ReverbMix, reverb, true);
}

/// Game.TestPreset("None");
/// Game.TestPreset("Underwater");
/// Game.TestPreset("OnThePhone");
public static exec func TestPreset(game: GameInstance, preset: String) {
    let value: Int32;
    switch preset {
        case "OnThePhone":
            value = EnumInt<Preset>(Preset.OnThePhone);
            break;
        case "Underwater":
            value = EnumInt<Preset>(Preset.Underwater);
            break;
        default:
            value = EnumInt<Preset>(Preset.None);
            break;
    }
    GameInstance.GetBlackboardSystem(game)
    .Get(GetAllBlackboardDefs().Audioware_Settings)
    .SetInt(GetAllBlackboardDefs().Audioware_Settings.AudioPreset, value, true);
}

/// Game.TestAutoRegisterEmitters("VendingMachine");
public static exec func TestAutoRegisterEmitters(game: GameInstance, className: String) {
    let cname = StringToName(className);
    GameInstance.GetAudioSystem(game).AutoRegisterEmitters(cname);
}
