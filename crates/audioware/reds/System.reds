module Audioware

public class AudiowareSystem extends ScriptableSystem {}

class LifecycleSubSystem extends ScriptableSystem {
    private let onMenu: ref<CallbackHandle>;
    private func OnAttach() -> Void {
        OnGameSystemAttach();
        this.onMenu = GameInstance.GetBlackboardSystem(this.GetGameInstance())
        .Get(GetAllBlackboardDefs().UI_System)
        .RegisterListenerBool(GetAllBlackboardDefs().UI_System.IsInMenu, this, n"OnMenu");
    }
    private func OnDetach() -> Void {
        OnGameSystemDetach();
        if IsDefined(this.onMenu) {
            GameInstance.GetBlackboardSystem(this.GetGameInstance())
            .Get(GetAllBlackboardDefs().UI_System)
            .UnregisterListenerBool(GetAllBlackboardDefs().UI_System.IsInMenu, this.onMenu);
        }
    }
    private final func OnPlayerAttach(request: ref<PlayerAttachRequest>) -> Void {
        OnGameSystemPlayerAttach();
    }
    private final func OnPlayerDetach(request: ref<PlayerDetachRequest>) -> Void {
        OnGameSystemPlayerDetach();
    }
    protected cb func OnMenu(value: Bool) -> Bool {
        OnUIMenu(value);
        return value;
    }
}

class VolumeSubSystem extends ScriptableSystem {
    private let settingsListener: ref<VolumeSettingsListener>;
    private func OnAttach() -> Void {
        this.settingsListener = new VolumeSettingsListener();
        this.settingsListener.Initialize(this.GetGameInstance());
        this.settingsListener.Start();
    }
    private func OnDetach() -> Void {
        this.settingsListener = null;
    }
}

class SettingsSubSystem extends ScriptableSystem {
    private let playerReverbListener: ref<CallbackHandle>;
    private let playerPresetListener: ref<CallbackHandle>;
    private let swimListener: ref<CallbackHandle>;
    private func OnAttach() -> Void {
        let system: ref<BlackboardSystem> = GameInstance.GetBlackboardSystem(this.GetGameInstance());
        let definitions: ref<AllBlackboardDefinitions> = GetAllBlackboardDefs();
        let audio: ref<IBlackboard> = system.Get(definitions.Audioware_Settings);
        this.playerReverbListener = audio
        .RegisterListenerFloat(definitions.Audioware_Settings.ReverbMix, this, n"OnPlayerReverb", false);
        this.playerPresetListener = audio
        .RegisterListenerInt(definitions.Audioware_Settings.AudioPreset, this, n"OnPlayerPreset", false);
    }
    private func OnDetach() -> Void {
        let system: ref<BlackboardSystem> = GameInstance.GetBlackboardSystem(this.GetGameInstance());
        let definitions: ref<AllBlackboardDefinitions> = GetAllBlackboardDefs();
        let audio: ref<IBlackboard> = system.Get(definitions.Audioware_Settings);
        if IsDefined(this.playerReverbListener) {
            audio.UnregisterListenerFloat(definitions.Audioware_Settings.ReverbMix, this.playerReverbListener);
        }
        if IsDefined(this.playerPresetListener) {
            audio.UnregisterListenerInt(definitions.Audioware_Settings.AudioPreset, this.playerPresetListener);
        }
    }
    private final func OnPlayerAttach(request: ref<PlayerAttachRequest>) -> Void {
        let player = request.owner as PlayerPuppet;
        if IsDefined(player) {
            let psm: ref<IBlackboard> = player.GetPlayerStateMachineBlackboard();
            this.swimListener = psm.RegisterListenerInt(GetAllBlackboardDefs().PlayerStateMachine.Swimming, this, n"OnSwim", true);
        }
    }
    private final func OnPlayerDetach(request: ref<PlayerDetachRequest>) -> Void {
        let player = GameInstance.FindEntityByID(this.GetGameInstance(), request.ownerID) as PlayerPuppet;
        if IsDefined(player) {
            let psm: ref<IBlackboard> = player.GetPlayerStateMachineBlackboard();
            psm.UnregisterListenerInt(GetAllBlackboardDefs().PlayerStateMachine.Swimming, this.swimListener);
        }
    }
    protected cb func OnPlayerReverb(value: Float) -> Bool {
        SetReverbMix(value);
    }
    protected cb func OnPlayerPreset(value: Int32) -> Bool {
        let preset = IntEnum<Preset>(value);
        SetPreset(preset);
    }
    protected cb func OnSwim(value: Int32) -> Bool {
        let state = IntEnum<gamePSMSwimming>(value);
        let diving = Equals(state, gamePSMSwimming.Diving);
        SetPreset(diving ? Preset.Underwater : Preset.None);
    }
}

class SubtitleSubSystem extends ScriptableSystem {
    private let subtitleDelayID: DelayID;
    private let subtitleRemaining: Float = 0.0;
    private let subtitleLine: scnDialogLineData;

    private let menuListener: ref<CallbackHandle>;

    private func OnAttach() -> Void {
        let system: ref<BlackboardSystem> = GameInstance.GetBlackboardSystem(this.GetGameInstance());
        let definitions: ref<AllBlackboardDefinitions> = GetAllBlackboardDefs();
        let ui: ref<IBlackboard> = system.Get(definitions.UI_System);
        this.menuListener = ui
        .RegisterListenerBool(definitions.UI_System.IsInMenu, this, n"OnInMenu");
    }

    private func OnDetach() -> Void {
        this.CancelHideSubtitle();
        let system: ref<BlackboardSystem> = GameInstance.GetBlackboardSystem(this.GetGameInstance());
        let definitions: ref<AllBlackboardDefinitions> = GetAllBlackboardDefs();
        if IsDefined(this.menuListener) {
            let ui: ref<IBlackboard> = system.Get(definitions.UI_System);
            ui.UnregisterListenerBool(definitions.UI_System.IsInMenu, this.menuListener);
            this.menuListener = null;
        }
    }

    public func DelayHideSubtitle(line: scnDialogLineData, duration: Float) {
        let callback: ref<HideSubtitleCallback> = new HideSubtitleCallback();
        callback.line = line;
        this.subtitleLine = line;
        this.subtitleDelayID = GameInstance
        .GetDelaySystem(GetGameInstance())
        .DelayCallback(callback, duration);
    }

    public func CancelHideSubtitle() {
        if NotEquals(this.subtitleDelayID, GetInvalidDelayID()) {
            GameInstance
            .GetDelaySystem(this.GetGameInstance())
            .CancelCallback(this.subtitleDelayID);
            this.subtitleRemaining = 0.0;
            this.subtitleDelayID = GetInvalidDelayID();
        }
    }

    public func PauseHideSubtitleCallback() -> Void {
        if NotEquals(this.subtitleDelayID, GetInvalidDelayID()) {
            this.subtitleRemaining = GameInstance.GetDelaySystem(GetGameInstance())
            .GetRemainingDelayTime(this.subtitleDelayID);
            GameInstance.GetDelaySystem(GetGameInstance()).CancelCallback(this.subtitleDelayID);
        }
    }

    public func ResumeHideSubtitleCallback() -> Void {
        if this.subtitleRemaining >= 0.3 {
            let callback: ref<HideSubtitleCallback> = new HideSubtitleCallback();
            callback.line = this.subtitleLine;
            this.subtitleDelayID = GameInstance
            .GetDelaySystem(this.GetGameInstance())
            .DelayCallback(callback, this.subtitleRemaining);
        }
    }

    protected cb func OnInMenu(value: Bool) -> Bool {
        if value {
            this.PauseHideSubtitleCallback();
        } else {
            this.ResumeHideSubtitleCallback();
        }
    }

    public final func CancelDelay() -> Void {
        if NotEquals(this.subtitleDelayID, GetInvalidDelayID()) {
            GameInstance
            .GetDelaySystem(this.GetGameInstance())
            .CancelCallback(this.subtitleDelayID);
        }
    }

    public final static func GetInstance(game: GameInstance) -> ref<SubtitleSubSystem> = GameInstance.GetScriptableSystemsContainer(game).Get(n"Audioware.SubtitleSubSystem") as SubtitleSubSystem;
}
