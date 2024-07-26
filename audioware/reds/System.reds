module Audioware

public native func TestPlay() -> Void;

public class AudiowareSystem extends ScriptableSystem {
    private let menuListener: ref<CallbackHandle>;
    private let playerReverbListener: ref<CallbackHandle>;
    private let playerPresetListener: ref<CallbackHandle>;

    private func OnAttach() -> Void {
        LOG("on attach: AudiowareSystem");
        let system: ref<BlackboardSystem> = GameInstance.GetBlackboardSystem(this.GetGameInstance());
        let definitions: ref<AllBlackboardDefinitions> = GetAllBlackboardDefs();
        let ui: ref<IBlackboard> = system.Get(definitions.UI_System);
        this.menuListener = ui.RegisterListenerBool(definitions.UI_System.IsInMenu, this, n"OnInMenu");
        let audio: ref<IBlackboard> = system.Get(definitions.Audioware_Settings);
        this.playerReverbListener = audio
        .RegisterListenerFloat(definitions.Audioware_Settings.PlayerReverb, this, n"OnPlayerReverb", false);
        this.playerPresetListener = audio
        .RegisterListenerInt(definitions.Audioware_Settings.PlayerPreset, this, n"OnPlayerPreset", false);
    }
    private func OnDetach() -> Void {
        LOG("on detach: AudiowareSystem");
        ClearEmitters();
        let system: ref<BlackboardSystem> = GameInstance.GetBlackboardSystem(this.GetGameInstance());
        let definitions: ref<AllBlackboardDefinitions> = GetAllBlackboardDefs();
        if IsDefined(this.menuListener) {
            let ui: ref<IBlackboard> = system.Get(definitions.UI_System);
            ui.UnregisterListenerBool(definitions.UI_System.IsInMenu, this.menuListener);
            this.menuListener = null;
        }
        let audio: ref<IBlackboard> = system.Get(definitions.Audioware_Settings);
        if IsDefined(this.playerReverbListener) {
            audio.UnregisterListenerFloat(definitions.Audioware_Settings.PlayerReverb, this.playerReverbListener);
        }
        if IsDefined(this.playerReverbListener) {
            audio.UnregisterListenerInt(definitions.Audioware_Settings.PlayerPreset, this.playerPresetListener);
        }
    }
    private final func OnPlayerAttach(request: ref<PlayerAttachRequest>) -> Void {
        LOG("on player attach: AudiowareSystem");
        SetGameState(GameState.InGame);
        TestPlay();
    }
    private final func OnPlayerDetach(request: ref<PlayerDetachRequest>) -> Void {
        LOG("on player detach: AudiowareSystem");
        UnsetPlayerGender();
    }
    protected cb func OnInMenu(value: Bool) -> Bool {
        LOG(s"on \(value ? "enter" : "exit") menu: AudiowareSystem");
        SetGameState(value ? GameState.InMenu : GameState.InGame);
    }
    protected cb func OnPlayerReverb(value: Float) -> Bool {
        LOG(s"on player reverb changed (\(ToString(value))): AudiowareSystem");
        SetPlayerReverb(value);
    }
    protected cb func OnPlayerPreset(value: Int32) -> Bool {
        let preset = IntEnum<Preset>(value);
        LOG(s"on player preset changed (\(ToString(preset))): AudiowareSystem");
        SetPlayerPreset(preset);
    }
}
