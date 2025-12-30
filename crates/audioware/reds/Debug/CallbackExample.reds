import Audioware.*

public class AudioCallbackService extends ScriptableService {
    private let occlusions: ref<inkIntHashMap>;
    private cb func OnLoad() {
        FTLog(s"AudioCallbackService.OnLoad");
        let system = new AudioEventCallbackSystem();
        system.RegisterCallback(n"game_occlusion", this, n"OnOcclusion");
        system.RegisterStaticCallback(n"veh_tyre_condition", n"AudioCallbackService", n"OnTyreCondition");
        system.RegisterStaticCallback(n"game_window_in_focus", n"AudioCallbackService", n"OnWindowsInFocus");
        system.RegisterStaticCallback(n"default", n"AudioCallbackService", n"OnSetDefaultAppearance")
            .SetLifetime(AudioEventCallbackLifetime.Forever);
        system.RegisterStaticCallback(n"ph_metal_car", n"AudioCallbackService", n"OnMetalCar");
    }
    public cb func OnOcclusion(event: ref<SetParameterEvent>) {
        let hash = EntityID.GetHash(event.EntityID());
        let percent = event.ParamValue() * 100.;
        let value = Cast<Int32>(percent);
        if this.occlusions.KeyExist(Cast(hash)) {
            if this.occlusions.Get(Cast(hash)) == value {
                return;
            } else {
                this.occlusions.Set(Cast(hash), value);
            }
        } else {
            this.occlusions.Insert(Cast(hash), value);
        }
        FTLog(s"AudioCallbackService.OnOcclusion name_data: \(NameToString(event.ParamName())), float_data: \(event.ParamValue()), entity_id: \(EntityID.ToDebugString(event.EntityID())), emitter_name: \(NameToString(event.EmitterName())), wwise_id: \(event.WwiseID())");
    }
    public cb static func OnTyreCondition(event: ref<SetSwitchEvent>) {
         FTLog(s"AudioCallbackService.OnTyreCondition switch_name: \(NameToString(event.SwitchName())), switch_value: \(NameToString(event.SwitchValue())), switch_name_wwise_id: \(event.SwitchNameWwiseID())");
    }
    public cb static func OnWindowsInFocus(event: ref<SetGlobalParameterEvent>) {
         FTLog(s"AudioCallbackService.OnWindowsInFocus parameter_name: \(NameToString(event.Name())), parameter_value: \(event.Value()), wwise_id: \(event.WwiseID())");
    }
    public cb static func OnSetDefaultAppearance(event: ref<SetAppearanceNameEvent>) {
         FTLog(s"AudioCallbackService.OnSetDefaultAppearance parameter_name: \(NameToString(event.Name())), entity_id: \(EntityID.ToDebugString(event.EntityID()))");
    }
    public cb static func OnMetalCar(base: ref<SoundEvent>) {
        if base.IsA(n"Audioware.PlayExternalEvent") {
            let event = base as PlayExternalEvent;
            FTLog(s"AudioCallbackService.OnMetalCar [play external] event_name: \(NameToString(event.EventName())), entity_id: \(event.EntityID()), emitter_name: \(NameToString(event.EmitterName())), wwise_id: \(event.WwiseID()), external_resource_path: \(event.ExternalResourcePath())");
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
            FTLog(s"AudioCallbackService.OnMetalCar [play oneshot] event_name: \(NameToString(event.EventName())), entity_id: \(event.EntityID()), emitter_name: \(NameToString(event.EmitterName())), \(str), wwise_id: \(event.WwiseID())");
        } else {
            FTLog(s"AudioCallbackService.OnMetalCar class_name: \(NameToString(base.GetClassName()))");
        }
    }
}

public class MyFireSystem extends ScriptableSystem {
    // change for whichever weapon you're wielding
    public let weaponEventName: CName = n"w_gun_lmg_power_ma70_fire_single";
    private let fire: ref<AudioEventCallbackHandler>;
    private let add: ref<AudioEventCallbackHandler>;
    private let idle: ref<AudioEventCallbackHandler>;
    private final func OnPlayerAttach(request: ref<PlayerAttachRequest>) -> Void {
        let system = new AudioEventCallbackSystem();
        if !IsDefined(this.add) {
            this.add = system
                .RegisterCallback(this.weaponEventName, this, n"OnFireSMG")
                .AddTarget(EventTarget.ActionType(IntEnum<audioEventActionType>(Cast<Int32>(EnumValueFromName(n"audioEventActionType", n"AddContainerStreamingPrefetch")))))
                .AddTarget(EventTarget.ActionType(IntEnum<audioEventActionType>(Cast<Int32>(EnumValueFromName(n"audioEventActionType", n"RemoveContainerStreamingPrefetch")))));
        }
        if !IsDefined(this.fire) {
            this.fire = system
                .RegisterCallback(this.weaponEventName, this, n"OnFireSMG")
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
    private cb func OnFireSMG(event: ref<SoundEvent>) {
        if event.IsA(n"Audioware.PlayEvent") {
            let play = event as PlayEvent;
            FTLog(s"MyFireSystem.OnFireSMG on entity: \(EntityID.ToDebugString(play.EntityID())) \(play.EntityID()), emitter name: \(NameToString(play.EmitterName()))");
        }
        else if event.IsA(n"Audioware.AddContainerStreamingPrefetchEvent") {
            let prefetch = event as AddContainerStreamingPrefetchEvent;
            FTLog(s"MyFireSystem.OnFireSMG \(NameToString(event.GetClassName())) entity: \(EntityID.ToDebugString(prefetch.EntityID())), wwise_id: \(prefetch.WwiseID())");
        }
        else if event.IsA(n"Audioware.RemoveContainerStreamingPrefetchEvent") {
            let prefetch = event as RemoveContainerStreamingPrefetchEvent;
            FTLog(s"MyFireSystem.OnFireSMG \(NameToString(event.GetClassName())) entity: \(EntityID.ToDebugString(prefetch.EntityID())), wwise_id: \(prefetch.WwiseID())");
        }
        else {
            FTLog(s"MyFireSystem.OnFireSMG \(NameToString(event.GetClassName()))");
        }
    }
    private cb func OnStopIdleStart(event: ref<StopSoundEvent>) {
        FTLog(s"MyFireSystem.OnStopIdleStart => event_name: \(NameToString(event.SoundName())), entity_id: \(EntityID.ToDebugString(event.EntityID())), fade_out: \(event.FadeOut())");
    }
}