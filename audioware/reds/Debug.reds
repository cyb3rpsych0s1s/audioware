import Audioware.AudioSettingsExt
import Audioware.Tween
import Audioware.LinearTween
import Audioware.EmitterDistances
import Audioware.EmitterSettings
import Audioware.Preset
import Audioware.Audioware_SettingsDef
import Audioware.DummyLol

/// HotReload();
private native static func HotReload() -> Void;

/// Game.TestPlayExt("straight_outta_compton");
public static exec func TestPlayExt(game: GameInstance, name: String) {
    GameInstance.GetAudioSystemExt(game).Play(StringToName(name));
}
/// Game.TestPlayExtWithTween("straight_outta_compton");
public static exec func TestPlayExtWithTween(game: GameInstance, name: String) {
    let ext = new AudioSettingsExt();
    ext.fadeIn = LinearTween.Immediate(5.);
    let noId: EntityID;
    GameInstance.GetAudioSystemExt(game).Play(StringToName(name), noId, n"None", scnDialogLineType.Regular, ext);
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
    let emitterCName: CName = StringToName(emitterName);

    let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
    emitterID = target.GetEntityID();
    let registered = GameInstance.GetAudioSystemExt(game).RegisterEmitter(emitterID, emitterCName);
    FTLog(s"registered: \(registered)");
    if registered {
        GameInstance.GetAudioSystemExt(game).PlayOnEmitter(soundCName, emitterID, emitterCName);
    }
}

public class AutoEmittersSystem extends ScriptableSystem {
    private func OnAttach() {
        GameInstance.GetCallbackSystem().RegisterCallback(n"Input/Key", this, n"OnPressF1")
        .AddTarget(InputTarget.Key(EInputKey.IK_F1));
        GameInstance.GetCallbackSystem().RegisterCallback(n"Input/Key", this, n"OnPressF2")
        .AddTarget(InputTarget.Key(EInputKey.IK_F2));
    }
    private cb func OnPressF1(evt: ref<KeyInputEvent>) {
        if NotEquals(evt.GetAction(), EInputAction.IACT_Release) { return; }
        let dummy = new DummyLol();
        dummy.Hi();
        let sounds = [ 
            n"coco_caline",
            n"god_love_us", 
            n"copacabana",
            n"dok_mai_gab_jeh_gan", 
            n"ton",
            n"dimanche_aux_goudes",
            n"feel_good_inc",
            n"straight_outta_compton",
            n"welcome_to_brownsville",
            n"sultans_of_swing",
            n"ghetto_vet",
            n"get_off_the_ground"
        ];
        let eventName = sounds[RandRange(0, ArraySize(sounds) -1)];
        let tween = new LinearTween();
        tween.duration = 1.0;
        let ext = new AudioSettingsExt();
        ext.fadeIn = tween;
        let emitterID: EntityID;
        let emitterCName: CName = n"DummyTest";

        let game = this.GetGameInstance();
        let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
        if !IsDefined(target) { return; }
        emitterID = target.GetEntityID();
        if GameInstance.GetAudioSystemExt(game).RegisterEmitter(emitterID, emitterCName) {
            FTLog(s"play on emitter: AutoEmittersSystem");
            GameInstance.GetAudioSystemExt(game).PlayOnEmitter(eventName, emitterID, emitterCName, ext);
        }
    }
    private cb func OnPressF2(evt: ref<KeyInputEvent>) {
        if NotEquals(evt.GetAction(), EInputAction.IACT_Release) { return; }
        let game = this.GetGameInstance();
        let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
        let there = target.GetWorldPosition();
        FTLog(s"target position: [x: \(there.X), y: \(there.Y)], z: \(there.Z)");
        let v = GetPlayer(game);
        let here = v.GetWorldPosition();
        FTLog(s"V position:      [x: \(here.X), y: \(here.Y)], z: \(here.Z)");
        let diff = new Vector3(AbsF(there.X - here.X), AbsF(there.Y - here.Y), AbsF(there.Z - here.Z));
        FTLog(s"difference:      [x: \(diff.X), y: \(diff.Y)], z: \(diff.Z)");
    }
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
