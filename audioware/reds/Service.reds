module Audioware

class AudiowareService extends ScriptableService {
    private cb func OnLoad() {
        GameInstance.GetCallbackSystem()
            .RegisterCallback(n"Session/Ready", this, n"OnSessionReady");
        GameInstance.GetCallbackSystem()
            .RegisterCallback(n"Entity/Attached", this, n"OnPlayerSpawn")
            .AddTarget(EntityTarget.Type(n"PlayerPuppet"));
        GameInstance.GetCallbackSystem()
            .RegisterCallback(n"Entity/Uninitialize", this, n"OnPlayerDespawn")
            .AddTarget(EntityTarget.Type(n"PlayerPuppet"));
    }

    private cb func OnSessionReady(event: ref<GameSessionEvent>) {
        FTLog(AsRef("on session ready: AudiowareService"));
        // TestPlay();
    }

    private cb func OnPlayerSpawn(event: ref<EntityLifecycleEvent>) {
        let v = event.GetEntity();
        if IsDefined(v) {
            RegisterListener(v.GetEntityID());
        }
    }

    private cb func OnPlayerDespawn(event: ref<EntityLifecycleEvent>) {
        let v = event.GetEntity();
        if IsDefined(v) {
            UnregisterListener(v.GetEntityID());
        }
    }
}