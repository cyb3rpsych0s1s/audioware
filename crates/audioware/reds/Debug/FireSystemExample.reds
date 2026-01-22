import Audioware.*

public class MyFireSystem extends ScriptableSystem {
    // change for whichever weapon you're wielding
    public let weaponEventName: CName = n"w_gun_lmg_power_ma70_fire_single";
    private let fire: ref<AudioEventCallbackHandler>;
    private let add: ref<AudioEventCallbackHandler>;
    private let idle: ref<AudioEventCallbackHandler>;
    private final func OnPlayerAttach(request: ref<PlayerAttachRequest>) -> Void {
        if !EnableAudiowareFireSystemExample() { return; }
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
                .AddTarget(EmitterTarget.EmitterName(n"firearm_emitter"));
        }
        if !IsDefined(this.idle) {
            this.idle = system
                .RegisterCallback(n"idleStart", this, n"OnStopIdleStart")
                .AddTarget(EventTarget.ActionType(audioEventActionType.StopSound));
        }
    }
    public final func OnPlayerDetach(player: ref<PlayerPuppet>) -> Void {
        if !EnableAudiowareFireSystemExample() { return; }
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
