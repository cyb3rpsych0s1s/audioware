module Audioware

public native func TestPlay() -> Void;

public class AudiowareSystem extends ScriptableSystem {
    private let settingsListener: ref<VolumeSettingsListener>;
    private let menuListener: ref<CallbackHandle>;
    private let playerReverbListener: ref<CallbackHandle>;
    private let playerPresetListener: ref<CallbackHandle>;
    private let swimListener: ref<CallbackHandle>;

    private let subtitleDelayID: DelayID;
    private let subtitleRemaining: Float = 0.0;
    private let subtitleLine: scnDialogLineData;

    private func OnAttach() -> Void {
        LOG("on attach: AudiowareSystem");
        let system: ref<BlackboardSystem> = GameInstance.GetBlackboardSystem(this.GetGameInstance());
        let definitions: ref<AllBlackboardDefinitions> = GetAllBlackboardDefs();
        let ui: ref<IBlackboard> = system.Get(definitions.UI_System);
        this.menuListener = ui
        .RegisterListenerBool(definitions.UI_System.IsInMenu, this, n"OnInMenu");
        let audio: ref<IBlackboard> = system.Get(definitions.Audioware_Settings);
        this.playerReverbListener = audio
        .RegisterListenerFloat(definitions.Audioware_Settings.PlayerReverb, this, n"OnPlayerReverb", false);
        this.playerPresetListener = audio
        .RegisterListenerInt(definitions.Audioware_Settings.PlayerPreset, this, n"OnPlayerPreset", false);

        this.settingsListener = new VolumeSettingsListener();
        this.settingsListener.Initialize(this.GetGameInstance());
    }
    private func OnDetach() -> Void {
        LOG("on detach: AudiowareSystem");
        this.CancelHideSubtitle();
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
        if IsDefined(this.playerPresetListener) {
            audio.UnregisterListenerInt(definitions.Audioware_Settings.PlayerPreset, this.playerPresetListener);
        }
    }
    private final func OnPlayerAttach(request: ref<PlayerAttachRequest>) -> Void {
        LOG("on player attach: AudiowareSystem");
        SetGameState(GameState.InGame);
        TestPlay();

        let player = request.owner as PlayerPuppet;
        if IsDefined(player) {
            let psm: ref<IBlackboard> = player.GetPlayerStateMachineBlackboard();
            this.swimListener = psm.RegisterListenerInt(GetAllBlackboardDefs().PlayerStateMachine.Swimming, this, n"OnSwim", true);
        }
        this.settingsListener.Start();
    }
    private final func OnPlayerDetach(request: ref<PlayerDetachRequest>) -> Void {
        LOG("on player detach: AudiowareSystem");
        UnsetPlayerGender();

        let player = GameInstance.FindEntityByID(this.GetGameInstance(), request.ownerID) as PlayerPuppet;
        if IsDefined(player) {
            let psm: ref<IBlackboard> = player.GetPlayerStateMachineBlackboard();
            psm.UnregisterListenerInt(GetAllBlackboardDefs().PlayerStateMachine.Swimming, this.swimListener);
        }

        this.settingsListener = null;
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
        LOG(s"on \(value ? "enter" : "exit") menu: AudiowareSystem");
        SetGameState(value ? GameState.InMenu : GameState.InGame);
        if value {
            Pause();
            this.PauseHideSubtitleCallback();
        } else {
            Resume();
            this.ResumeHideSubtitleCallback();
        }
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
    protected cb func OnSwim(value: Int32) -> Bool {
        let state = IntEnum<gamePSMSwimming>(value);
        let diving = Equals(state, gamePSMSwimming.Diving);
        SetPlayerPreset(diving ? Preset.Underwater : Preset.None);
    }

    public final static func GetInstance(game: GameInstance) -> ref<AudiowareSystem> {
        return GameInstance
        .GetScriptableSystemsContainer(game)
        .Get(n"Audioware.AudiowareSystem") as AudiowareSystem;
    }
}

public class HideSubtitleCallback extends DelayCallback {
    public let line: scnDialogLineData;
    public func Call() -> Void {
        if !IsDefined(this.line.speaker) { return; }
        let game = this.line.speaker.GetGame();
        GameInstance
        .GetDelaySystem(game)
        .CancelCallback(AudiowareSystem.GetInstance(game).subtitleDelayID);
        let board: ref<IBlackboard> = GameInstance.GetBlackboardSystem(game).Get(GetAllBlackboardDefs().UIGameData);
        board.SetVariant(GetAllBlackboardDefs().UIGameData.HideDialogLine, [this.line.id], true);
    }
}
