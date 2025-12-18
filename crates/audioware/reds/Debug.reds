import Audioware.AudioSettingsExt
import Audioware.Tween
import Audioware.LinearTween
import Audioware.EmitterDistances
import Audioware.EmitterSettings
import Audioware.Preset
import Audioware.Audioware_SettingsDef
import Audioware.*

/// Game.TestPlay("as_if_I_didnt_know_already");
public exec func TestPlay(game: GameInstance, name: String) {
    GameInstance.GetAudioSystem(game).Play(StringToName(name), GetPlayer(game).GetEntityID(), n"V");
}
/// Game.TestPlayExt("straight_outta_compton");
public exec func TestPlayExt(game: GameInstance, name: String) {
    GameInstance.GetAudioSystemExt(game).Play(StringToName(name));
}
/// Game.TestPlayExtWithTween("straight_outta_compton");
public exec func TestPlayExtWithTween(game: GameInstance, name: String) {
    let ext = new AudioSettingsExt();
    ext.fadeIn = LinearTween.Immediate(5.);
    let noId: EntityID;
    GameInstance.GetAudioSystemExt(game).Play(StringToName(name), noId, n"None", scnDialogLineType.Regular, ext);
}
/// Game.TestStop("straight_outta_compton");
public exec func TestStop(game: GameInstance, name: String) {
    GameInstance.GetAudioSystemExt(game).Stop(StringToName(name));
}
/// Game.TestRegisterEmitter();
public exec func TestRegisterEmitter(game: GameInstance) {
    let emitterID: EntityID;
    let emitterName: CName;

    let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
    emitterID = target.GetEntityID();
    emitterName = n"Jean-Guy";
    let added = GameInstance.GetAudioSystemExt(game).RegisterEmitter(emitterID, n"Audioware", emitterName);
    FTLog(s"registered? \(added)");
}
/// Game.TestUnregisterEmitter();
public exec func TestUnregisterEmitter(game: GameInstance) {
    let emitterID: EntityID;

    let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
    emitterID = target.GetEntityID();
    let added = GameInstance.GetAudioSystemExt(game).UnregisterEmitter(emitterID, n"Audioware");
    FTLog(s"unregistered? \(added)");
}

/// Game.TestPlayOnEmitter("straight_outta_compton", "Eazy-E");
public exec func TestPlayOnEmitter(game: GameInstance, soundName: String, opt emitterName: String) {
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
public exec func TestPlayOverThePhone(game: GameInstance, name: String) {
    let cname = StringToName(name);
    GameInstance.GetAudioSystemExt(game).PlayOverThePhone(cname, n"Vik", n"Male");
}

/// Game.TestReverb(1.0);
/// Game.TestReverb(0.0);
public exec func TestReverb(game: GameInstance, reverb: Float) {
    GameInstance.GetBlackboardSystem(game)
    .Get(GetAllBlackboardDefs().Audioware_Settings)
    .SetFloat(GetAllBlackboardDefs().Audioware_Settings.ReverbMix, reverb, true);
}

/// Game.TestPreset("None");
/// Game.TestPreset("Underwater");
/// Game.TestPreset("OnThePhone");
public exec func TestPreset(game: GameInstance, preset: String) {
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
public exec func TestSupportedLanguages(game: GameInstance) {
    let languages = SupportedLanguages();
    if ArraySize(languages) == 0 { FTLog(s"banks do not contain entries for any language"); }
    else {
        for language in languages {
            FTLog(s"banks contain entries for \(NameToString(language))");
        }
    }
}

/// Game.TestVersion();
public exec func TestVersion(game: GameInstance) {
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
public exec func TestPreset(game: GameInstance, preset: String) {
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

public exec func TestHalfVolume(game: GameInstance)   { TestSpecificVolume(game, 0.5); }
public exec func TestDoubleVolume(game: GameInstance) { TestSpecificVolume(game, 2.0); }
public exec func TestNormalVolume(game: GameInstance) { TestSpecificVolume(game, 1.0); }

public exec func TestSpecificVolume(game: GameInstance, amplitude: Float) {
    let none: EntityID;
    let ext = new AudioSettingsExt();
    ext.volume = amplitude;
    GameInstance.GetAudioSystemExt(game).Play(n"straight_outta_compton", none, n"None", scnDialogLineType.None, ext);
}

public exec func StopTestVolume(game: GameInstance) {
    GameInstance.GetAudioSystemExt(game).Stop(n"straight_outta_compton");
}

public class AudioMuteService extends ScriptableService {
    private let replaced: Bool;
    private let handler: ref<AudioEventCallbackHandler>;
    private cb func OnLoad() {
        FTLog(s"AudioMuteService.OnLoad");
        let system = new AudioEventCallbackSystem();
        this.handler = system.RegisterCallback(n"Mixing_Output_Cinema", this, n"OnSfxMusicStart")
            .AddTarget(EventTarget.ActionType(audioEventActionType.Play));
        //system.RegisterStaticCallback(n"game_occlusion", n"AudioMuteService", n"OnOcclusion");
        //system.RegisterStaticCallback(n"veh_tyre_condition", n"AudioMuteService", n"OnTyreCondition");
        //system.RegisterStaticCallback(n"game_window_in_focus", n"AudioMuteService", n"OnWindowsInFocus");
        system.RegisterStaticCallback(n"default", n"AudioMuteService", n"OnSetDefaultAppearance")
            .SetLifetime(AudioEventCallbackLifetime.Forever);
        system.RegisterStaticCallback(n"ph_metal_car", n"AudioMuteService", n"OnMetalCar");
        let manager = new AudioEventManager();
        manager.Mute(n"cp_bumpers_temp_sfx_music_start");
        manager.MuteSpecific(n"cp_intro_temp_sfx_music_start", audioEventActionType.Play);
        manager.MuteSpecific(n"cp_intro_temp_sfx_music_stop", audioEventActionType.StopSound);
    }
    private cb func OnSfxMusicStart(evt: ref<SoundEvent>) {
        FTLog(s"AudioMuteService.OnSfxMusicStart");
        if !evt.IsA(n"Audioware.PlayEvent") {
            return;
        }
        let event = evt as PlayEvent;
        if !this.replaced {
            FTLog(s"PlayEvent event_name: \(NameToString(event.EventName())), entity_id: \(EntityID.ToDebugString(event.EntityID())), emitter_name: \(NameToString(event.EmitterName())), wwise_id: \(event.WwiseID())");
            //let audioware = new AudioSystemExt();
            //audioware.Play(n"intro_du_mordor");
            this.replaced = true;
            if IsDefined(this.handler) && this.handler.IsRegistered() {
                this.handler.Unregister();
            }
        }
    }
    public cb static func OnOcclusion(event: ref<SetParameterEvent>) {
        //FTLog(s"AudioMuteService.OnOcclusion switch_name: \(NameToString(event.SwitchName())), switch_value: \(event.SwitchValue()), entity_id: \(EntityID.ToDebugString(event.EntityID())), emitter_name: \(NameToString(event.EmitterName())), wwise_id: \(event.WwiseID())");
    }
    public cb static func OnTyreCondition(event: ref<SetSwitchEvent>) {
        //FTLog(s"AudioMuteService.OnTyreCondition switch_name: \(NameToString(event.SwitchName())), switch_value: \(NameToString(event.SwitchValue())), entity_id: \(EntityID.ToDebugString(event.EntityID())), emitter_name: \(NameToString(event.EmitterName())), switch_name_wwise_id: \(event.SwitchNameWwiseID())");
    }
    public cb static func OnWindowsInFocus(event: ref<SetGlobalParameterEvent>) {
        //FTLog(s"AudioMuteService.OnWindowsInFocus parameter_name: \(NameToString(event.Name())), parameter_value: \(event.Value()), wwise_id: \(event.WwiseID())");
    }
    public cb static func OnSetDefaultAppearance(event: ref<SetAppearanceNameEvent>) {
        //FTLog(s"AudioMuteService.OnSetDefaultAppearance parameter_name: \(NameToString(event.Name())), entity_id: \(EntityID.ToDebugString(event.EntityID())), emitter_name: \(NameToString(event.EmitterName())), metadata_name: \(NameToString(event.MetadataName()))");
    }
    public cb static func OnMetalCar(base: ref<SoundEvent>) {
        if base.IsA(n"Audioware.PlayExternalEvent") {
            let event = base as PlayExternalEvent;
            FTLog(s"AudioMuteService.OnMetalCar [play external] event_name: \(NameToString(event.EventName())), entity_id: \(event.EntityID()), emitter_name: \(NameToString(event.EmitterName())), wwise_id: \(event.WwiseID()), external_resource_path: \(event.ExternalResourcePath())");
        } else if base.IsA(n"Audioware.PlayOneShotEvent") {
            let event = base as PlayOneShotEvent;
            let parameters = event.Params();
            let str = "params: [";
            let first = true;
            for p in parameters {
                if !first { str += ", "; }
                str += s"\(NameToString(p.name)) = \(ToString(p.value))";
                first = false;
            }
            str += "]";
            str += ", ";
            str += "switches: [";
            let switches = event.Switches();
            first = true;
            for s in switches {
                if !first { str += ", "; }
                str += s"\(NameToString(s.name)) = \(NameToString(s.value))";
                first = false;
            }
            str += "]";
            FTLog(s"AudioMuteService.OnMetalCar [play oneshot] event_name: \(NameToString(event.EventName())), entity_id: \(event.EntityID()), emitter_name: \(NameToString(event.EmitterName())), \(str), wwise_id: \(event.WwiseID())");
        } else {
            FTLog(s"AudioMuteService.OnMetalCar class_name: \(NameToString(base.GetClassName()))");
        }
    }
}

public class MyFireSystem extends ScriptableSystem {
    private let fire: ref<AudioEventCallbackHandler>;
    private let add: ref<AudioEventCallbackHandler>;
    private let idle: ref<AudioEventCallbackHandler>;
    private final func OnPlayerAttach(request: ref<PlayerAttachRequest>) -> Void {
        let system = new AudioEventCallbackSystem();
        if !IsDefined(this.add) {
            this.add = system
                .RegisterCallback(n"w_gun_lmg_power_ma70_fire_single", this, n"OnFireSMG")
                .AddTarget(EventTarget.ActionType(IntEnum<audioEventActionType>(Cast<Int32>(EnumValueFromName(n"audioEventActionType", n"AddContainerStreamingPrefetch")))))
                .AddTarget(EventTarget.ActionType(IntEnum<audioEventActionType>(Cast<Int32>(EnumValueFromName(n"audioEventActionType", n"RemoveContainerStreamingPrefetch")))));
        }
        if !IsDefined(this.fire) {
            this.fire = system
                .RegisterCallback(n"w_gun_lmg_power_ma70_fire_single", this, n"OnFireSMG")
                .AddTarget(EntityTarget.EmitterName(n"firearm_emitter"));
        }
        if !IsDefined(this.idle) {
            this.idle = system
                .RegisterCallback(n"idleStart", this, n"OnStopIdleStart")
                .AddTarget(EventTarget.ActionType(audioEventActionType.StopSound));
        }
    }
    public final func OnPlayerDetach(player: ref<PlayerPuppet>) -> Void {
        if IsDefined(this.add) && this.add.IsRegistered() {
            this.add.Unregister();
            this.add = null;
        }
        if IsDefined(this.fire) && this.fire.IsRegistered() {
            this.fire.Unregister();
            this.fire = null;
        }
        if IsDefined(this.idle) && this.idle.IsRegistered() {
            this.idle.Unregister();
            this.idle = null;
        }
    }
    private cb func OnFireSMG(event: ref<WwiseEvent>) {
        if event.IsA(n"Audioware.PlayEvent") {
            let play = event as PlayEvent;
            FTLog(s"MyFireSystem.OnFireSMG on entity: \(EntityID.ToDebugString(play.EntityID())) \(play.EntityID()), emitter name: \(NameToString(play.EmitterName()))");
        }
        else if event.IsA(n"Audioware.AddContainerStreamingPrefetchEvent") {
            let prefetch = event as AddContainerStreamingPrefetchEvent;
            FTLog(s"MyFireSystem.OnFireSMG \(NameToString(event.GetClassName())) entity: \(EntityID.ToDebugString(prefetch.EntityID())), emitter name: \(NameToString(prefetch.EmitterName())), wwise_id: \(prefetch.WwiseID())");
        }
        else if event.IsA(n"Audioware.RemoveContainerStreamingPrefetchEvent") {
            let prefetch = event as RemoveContainerStreamingPrefetchEvent;
            FTLog(s"MyFireSystem.OnFireSMG \(NameToString(event.GetClassName())) entity: \(EntityID.ToDebugString(prefetch.EntityID())), emitter name: \(NameToString(prefetch.EmitterName())), wwise_id: \(prefetch.WwiseID())");
        }
        else {
            FTLog(s"MyFireSystem.OnFireSMG \(NameToString(event.GetClassName()))");
        }
    }
    private cb func OnStopIdleStart(event: ref<StopSoundEvent>) {
        FTLog(s"MyFireSystem.OnStopIdleStart => event_name: \(NameToString(event.EventName())), entity_id: \(EntityID.ToDebugString(event.EntityID())), float_data: \(event.FloatData())");
    }
}
