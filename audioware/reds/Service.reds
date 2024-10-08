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
        // main menu (pre-game)
        GameInstance.GetCallbackSystem()
            .RegisterCallback(n"Resource/Ready", this, n"OnMainMenuResourceReady")
            .AddTarget(ResourceTarget.Path(r"base\\gameplay\\gui\\fullscreen\\main_menu\\pregame_menu.inkmenu"))
            .SetRunMode(CallbackRunMode.Once);

        this.RegisterModSettings();
    }

    private cb func OnUninitialize() {
        this.UnregisterModSettings();
    }

    private cb func OnSessionChange(event: ref<GameSessionEvent>) {
        switch event.GetEventName() {
            case n"Session/BeforeStart":
                DBG("on session before start: AudiowareService");
                break;
            case n"Session/Start":
                DBG("on session start: AudiowareService");
                SetGameState(GameState.Start);
                break;
            case n"Session/Ready":
                DBG("on session ready: AudiowareService");
                break;
            case n"Session/Pause":
                DBG("on session pause: AudiowareService");
                Pause();
                break;
            case n"Session/Resume":
                DBG("on session resume: AudiowareService");
                Resume();
                break;
            case n"Session/BeforeEnd":
                DBG("on session before end: AudiowareService");
                SetGameState(GameState.End);
                SetPreset(Preset.None);
                SetReverbMix(0.);
                Shutdown();
                break;
            case n"Session/End":
                DBG("on session end: AudiowareService");
                break;
            default:
                break;
        }
    }

    private cb func OnMainMenuResourceReady(event: ref<ResourceEvent>) {
        DBG("on main menu ready: AudiowareService");
    }

    public static func GetInstance() -> ref<AudiowareService> {
        return GameInstance
        .GetScriptableServiceContainer()
        .GetService(n"Audioware.AudiowareService") as AudiowareService;
    }

    // audio config

    @if(ModuleExists("ModSettingsModule"))
    private func RegisterModSettings() { ModSettings.RegisterListenerToModifications(this); }
    @if(ModuleExists("ModSettingsModule"))
    private func UnregisterModSettings() { ModSettings.UnregisterListenerToModifications(this); }

    @if(!ModuleExists("ModSettingsModule"))
    private func RegisterModSettings() -> Void {}
    @if(!ModuleExists("ModSettingsModule"))
    private func UnregisterModSettings() -> Void {}
    
    public func OnModSettingsChange() { this.RefreshConfig(); }
    public func RefreshConfig() -> Void {
        this.config = new AudiowareConfig();
    }
}
