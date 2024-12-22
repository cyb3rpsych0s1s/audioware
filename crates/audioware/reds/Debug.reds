import Audioware.AudioSettingsExt
import Audioware.Tween
import Audioware.LinearTween
import Audioware.EmitterDistances
import Audioware.EmitterSettings
import Audioware.Preset
import Audioware.Audioware_SettingsDef
import Audioware.*

/// HotReload();
private native static func HotReload() -> Void;

/// Game.TestPlay("as_if_I_didnt_know_already");
public static exec func TestPlay(game: GameInstance, name: String) {
    GameInstance.GetAudioSystem(game).Play(StringToName(name), GetPlayer(game).GetEntityID(), n"V");
}
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
    let added = GameInstance.GetAudioSystemExt(game).RegisterEmitter(emitterID, n"Audioware", emitterName);
    FTLog(s"registered? \(added)");
}
/// Game.TestUnregisterEmitter();
public static exec func TestUnregisterEmitter(game: GameInstance) {
    let emitterID: EntityID;
    let emitterName: CName;

    let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
    emitterID = target.GetEntityID();
    emitterName = n"Jean-Guy";
    let added = GameInstance.GetAudioSystemExt(game).UnregisterEmitter(emitterID, n"Audioware");
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

/// Game.TestPlayOverThePhone("as_if_I_didnt_know_already");
public static exec func TestPlayOverThePhone(game: GameInstance, name: String) {
    let cname = StringToName(name);
    GameInstance.GetAudioSystemExt(game).PlayOverThePhone(cname, n"Vik", n"Male");
}

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

/// Game.TestSupportedLanguages();
public static exec func TestSupportedLanguages(game: GameInstance) {
    let languages = SupportedLanguages();
    if ArraySize(languages) == 0 { FTLog(s"banks do not contain entries for any language"); }
    else {
        for language in languages {
            FTLog(s"banks contain entries for \(NameToString(language))");
        }
    }
}

/// Game.TestVersion();
public static exec func TestVersion(game: GameInstance) {
    let semver = GameInstance.GetAudioSystemExt(game).Version();
    FTLog(semver);
}

public class AutoEmittersSystem extends ScriptableSystem {
    private func OnAttach() {
        GameInstance.GetCallbackSystem().RegisterCallback(n"Input/Key", this, n"OnPressF1")
        .AddTarget(InputTarget.Key(EInputKey.IK_F1));
        GameInstance.GetCallbackSystem().RegisterCallback(n"Input/Key", this, n"OnPressF2")
        .AddTarget(InputTarget.Key(EInputKey.IK_F2));
        GameInstance.GetCallbackSystem().RegisterCallback(n"Input/Key", this, n"OnPressF3")
        .AddTarget(InputTarget.Key(EInputKey.IK_F3));
        GameInstance.GetCallbackSystem().RegisterCallback(n"Input/Key", this, n"OnPressF4")
        .AddTarget(InputTarget.Key(EInputKey.IK_F4));
    }
    private func RandomSong() -> CName {
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
        return eventName;
    }
    private func PlayOnEmitter(
        eventName: CName,
        emitterID: EntityID,
        emitterCName: CName,
        opt ext: ref<AudioSettingsExt>,
        opt settings: ref<EmitterSettings>) {
        let game = this.GetGameInstance();
        let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
        if !IsDefined(target) { return; }
        emitterID = target.GetEntityID();
        if GameInstance.GetAudioSystemExt(game).RegisterEmitter(emitterID, n"Audioware", emitterCName, settings) {
            FTLog(s"play on emitter: AutoEmittersSystem");
            GameInstance.GetAudioSystemExt(game).PlayOnEmitter(eventName, emitterID, n"Audioware", ext);
        }
    }
    private cb func OnPressF1(evt: ref<KeyInputEvent>) {
        if NotEquals(evt.GetAction(), EInputAction.IACT_Release) { return; }
        let eventName = this.RandomSong();
        let emitterID: EntityID;
        let emitterCName: CName = evt.IsShiftDown() ? n"None" : n"DummyTest";

        this.PlayOnEmitter(eventName, emitterID, emitterCName);
    }
    private cb func OnPressF2(evt: ref<KeyInputEvent>) {
        if NotEquals(evt.GetAction(), EInputAction.IACT_Release) { return; }
        let eventName = this.RandomSong();
        let emitterID: EntityID;
        let emitterCName: CName = evt.IsShiftDown() ? n"None" : n"DummyTest";

        let ext = new AudioSettingsExt();
        ext.fadeIn = LinearTween.Immediate(2.0);

        let settings = new EmitterSettings();
        settings.persistUntilSoundsFinish = true;

        this.PlayOnEmitter(eventName, emitterID, emitterCName, ext, settings);
    }
    private cb func OnPressF3(evt: ref<KeyInputEvent>) {
        if NotEquals(evt.GetAction(), EInputAction.IACT_Release) { return; }
        let emitterID: EntityID;
        let game = this.GetGameInstance();
        let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
        if !IsDefined(target) { return; }
        emitterID = target.GetEntityID();

        GameInstance.GetAudioSystemExt(this.GetGameInstance()).UnregisterEmitter(emitterID, n"Audioware");
    }
    private cb func OnPressF4(evt: ref<KeyInputEvent>) {
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
