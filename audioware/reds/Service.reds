module Audioware

class AudiowareService extends ScriptableService {
    private cb func OnLoad() {
        // game session state
        GameInstance.GetCallbackSystem()
            .RegisterCallback(n"Session/BeforeStart", this, n"OnSessionChange");
        GameInstance.GetCallbackSystem()
            .RegisterCallback(n"Session/Start", this, n"OnSessionChange");
        GameInstance.GetCallbackSystem()
            .RegisterCallback(n"Session/Ready", this, n"OnSessionChange");
        GameInstance.GetCallbackSystem()
            .RegisterCallback(n"Session/Pause", this, n"OnSessionChange");
        GameInstance.GetCallbackSystem()
            .RegisterCallback(n"Session/Resume", this, n"OnSessionChange");
        GameInstance.GetCallbackSystem()
            .RegisterCallback(n"Session/BeforeEnd", this, n"OnSessionChange");
        GameInstance.GetCallbackSystem()
            .RegisterCallback(n"Session/End", this, n"OnSessionChange");
        
        // spatial scene emitters
        GameInstance.GetCallbackSystem()
            .RegisterCallback(n"Entity/Attached", this, n"OnPlayerSpawn")
            .AddTarget(EntityTarget.Type(n"PlayerPuppet"));
        GameInstance.GetCallbackSystem()
            .RegisterCallback(n"Entity/Uninitialize", this, n"OnPlayerDespawn")
            .AddTarget(EntityTarget.Type(n"PlayerPuppet"));
    }

    private cb func OnSessionChange(event: ref<GameSessionEvent>) {
        switch event.GetEventName() {
            case n"Session/BeforeStart":
                FTLog(AsRef("on session before start: AudiowareService"));
                break;
            case n"Session/Start":
                FTLog(AsRef("on session start: AudiowareService"));
                SetGameState(GameState.Start);
                break;
            case n"Session/Ready":
                FTLog(AsRef("on session ready: AudiowareService"));
                break;
            case n"Session/Pause":
                FTLog(AsRef("on session pause: AudiowareService"));
                break;
            case n"Session/Resume":
                FTLog(AsRef("on session resume: AudiowareService"));
                break;
            case n"Session/BeforeEnd":
                FTLog(AsRef("on session before end: AudiowareService"));
                SetGameState(GameState.End);
                break;
            case n"Session/End":
                FTLog(AsRef("on session end: AudiowareService"));
                break;
            default:
                break;
        }
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