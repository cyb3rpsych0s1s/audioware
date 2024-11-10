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
    }
    
    private cb func OnSessionChange(event: ref<GameSessionEvent>) {
        switch event.GetEventName() {
            case n"Session/BeforeStart":
                OnGameSessionBeforeStart();
                break;
            case n"Session/Start":
                OnGameSessionStart();
                break;
            case n"Session/Ready":
                OnGameSessionReady();
                break;
            case n"Session/Pause":
                OnGameSessionPause();
                break;
            case n"Session/Resume":
                OnGameSessionResume();
                break;
            case n"Session/BeforeEnd":
                OnGameSessionBeforeEnd();
                break;
            case n"Session/End":
                OnGameSessionEnd();
                break;
            default:
                break;
        }
    }
}