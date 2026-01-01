import Audioware.*

public class PlayVoiceOnNPC extends ScriptableSystem {
    private final func OnPlayerAttach(request: ref<PlayerAttachRequest>) -> Void {
        GameInstance.GetCallbackSystem().RegisterCallback(n"Input/Key", this, n"OnPressF6")
            .AddTarget(InputTarget.Key(EInputKey.IK_F6));
    }
    private cb func OnPressF6(evt: ref<KeyInputEvent>) {
        if NotEquals(evt.GetAction(), EInputAction.IACT_Release) { return; }
        let emitterID: EntityID;
        let emitterCName: CName;
        let ext: ref<AudioSettingsExt>;
        let settings: ref<EmitterSettings>;
        
        let game = this.GetGameInstance();
        let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
        if !IsDefined(target) { return; }
        emitterID = target.GetEntityID();
        emitterCName = n"BigMcFly";
        if GameInstance.GetAudioSystemExt(game).RegisterEmitter(emitterID, n"Audioware", emitterCName, settings) {
            FTLog(s"play on emitter: PlayVoiceOnNPC");
            GameInstance.GetAudioSystemExt(game).PlayOnEmitter(n"hello_big", emitterID, n"Audioware", ext);
        }
    }
}