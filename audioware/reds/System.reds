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