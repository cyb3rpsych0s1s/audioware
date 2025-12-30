import Audioware.*

public class ChangeIntroService extends ScriptableService {
    private let replaced: Bool;
    private let handler: ref<AudioEventCallbackHandler>;
    private cb func OnLoad() {
        FTLog(s"ChangeIntroService.OnLoad");
        let system = new AudioEventCallbackSystem();
        this.handler = system
            .RegisterCallback(n"Mixing_Output_Cinema", this, n"OnSfxMusicStart")
            .AddTarget(EventTarget.ActionType(audioEventActionType.Play));
        let manager = new AudioEventManager();
        manager.Mute(n"cp_bumpers_temp_sfx_music_start");
        manager.MuteSpecific(n"cp_intro_temp_sfx_music_start", audioEventActionType.Play);
        manager.MuteSpecific(n"cp_intro_temp_sfx_music_stop", audioEventActionType.StopSound);
    }
    private cb func OnSfxMusicStart(event: ref<PlayEvent>) {
        FTLog(s"ChangeIntroService.OnSfxMusicStart");
        if !this.replaced {
            FTLog(s"PlayEvent event_name: \(NameToString(event.EventName())), entity_id: \(EntityID.ToDebugString(event.EntityID())), emitter_name: \(NameToString(event.EmitterName())), wwise_id: \(event.WwiseID())");
            // let audioware = new AudioSystemExt();
            // audioware.Play(n"intro_du_mordor");
            this.replaced = true;
            if IsDefined(this.handler) && this.handler.IsRegistered() {
                this.handler.Unregister();
            }
        }
    }
}