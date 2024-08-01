module Audioware

enum Registration {
    Failed = 0,
    Ready = 1,
    Postponed = 2,
}

class AudiowareService extends ScriptableService {
    private let config: ref<AudiowareConfig>;

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

        GameInstance.GetCallbackSystem()
            .UnregisterCallback(n"Entity/Uninitialize", this, n"OnDespawn");
        GameInstance.GetCallbackSystem()
            .UnregisterCallback(n"Entity/Attached", this, n"OnSpawn");

        this.RegisterOnLoad();
    }

    private cb func OnDespawn(event: ref<EntityLifecycleEvent>) {}
    private cb func OnSpawn(event: ref<EntityLifecycleEvent>) {}

    private cb func OnUninitialize() {
        this.UnregisterOnUninitialize();
    }

    private cb func OnSessionChange(event: ref<GameSessionEvent>) {
        switch event.GetEventName() {
            case n"Session/BeforeStart":
                LOG("on session before start: AudiowareService");
                break;
            case n"Session/Start":
                LOG("on session start: AudiowareService");
                SetGameState(GameState.Start);
                break;
            case n"Session/Ready":
                LOG("on session ready: AudiowareService");
                break;
            case n"Session/Pause":
                LOG("on session pause: AudiowareService");
                Pause();
                break;
            case n"Session/Resume":
                LOG("on session resume: AudiowareService");
                Resume();
                break;
            case n"Session/BeforeEnd":
                LOG("on session before end: AudiowareService");
                SetGameState(GameState.End);
                SetPreset(Preset.None);
                SetReverbMix(0.);
                Shutdown();
                break;
            case n"Session/End":
                LOG("on session end: AudiowareService");
                break;
            default:
                break;
        }
    }

    public static func GetInstance() -> ref<AudiowareService> {
        return GameInstance
        .GetScriptableServiceContainer()
        .GetService(n"Audioware.AudiowareService") as AudiowareService;
    }

    // audio config

    @if(ModuleExists("ModSettingsModule"))
    private func RegisterOnLoad() { ModSettings.RegisterListenerToModifications(this); }
    @if(ModuleExists("ModSettingsModule"))
    private func UnregisterOnUninitialize() { ModSettings.UnregisterListenerToModifications(this); }

    @if(!ModuleExists("ModSettingsModule"))
    private func RegisterOnLoad() -> Void {}
    @if(!ModuleExists("ModSettingsModule"))
    private func UnregisterOnUninitialize() -> Void {}
    
    public func OnModSettingsChange() { this.RefreshConfig(); }
    public func RefreshConfig() -> Void {
        this.config = new AudiowareConfig();
    }
}
