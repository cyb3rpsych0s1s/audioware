module Audioware

class AudiowareService extends ScriptableService {
    private let handler: ref<CallbackSystemHandler>;
    private let config: ref<AudiowareConfig>;
    private let isPreGame: Bool;

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
        
        // spatial scene
        this.handler = GameInstance.GetCallbackSystem()
            .RegisterCallback(n"Entity/Uninitialize", this, n"OnDespawn")
            .AddTarget(EntityTarget.Type(n"PlayerPuppet"));

        this.RegisterOnLoad();
    }

    private cb func OnUninitialize() {
        this.UnregisterOnUninitialize();
        this.handler = null;
    }

    private cb func OnSessionChange(event: ref<GameSessionEvent>) {
        this.isPreGame = event.IsPreGame();
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
                SetPlayerPreset(Preset.None);
                SetPlayerReverb(0.);
                Shutdown();
                break;
            case n"Session/End":
                LOG("on session end: AudiowareService");
                break;
            default:
                break;
        }
    }

    private cb func OnDespawn(event: ref<EntityLifecycleEvent>) {
        let entity = event.GetEntity();
        if !IsDefined(entity) { return; }
        if !entity.IsA(n"PlayerPuppet") {
            LOG("on emitter despawn: AudiowareService");
            UnregisterEmitter(entity.GetEntityID());
        } else { LOG("on player despawn: AudiowareService"); }
    }

    public func AddTarget(target: ref<CallbackSystemTarget>) {
        this.handler.AddTarget(target);
    }

    public func RemoveTarget(target: ref<CallbackSystemTarget>) {
        this.handler.RemoveTarget(target);
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

    public func IsPreGame() -> Bool { return this.isPreGame; }
}
