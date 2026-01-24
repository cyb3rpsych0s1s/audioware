import Audioware.*

public class AudioCallbackService extends ScriptableService {
    private let occlusions: ref<inkIntHashMap>;
    private cb func OnLoad() {
        if !EnableAudiowareCallbackExample() { return; }
        FTLog(s"AudioCallbackService.OnLoad");
        let system = new AudioEventCallbackSystem();
        system.RegisterCallback(n"game_occlusion", this, n"OnOcclusion");
        system.RegisterStaticCallback(n"veh_tyre_condition", n"AudioCallbackService", n"OnTyreCondition");
        //system.RegisterStaticCallback(n"game_window_in_focus", n"AudioCallbackService", n"OnWindowsInFocus");
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
