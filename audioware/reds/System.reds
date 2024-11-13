module Audioware

public class AudiowareSystem extends ScriptableSystem {
    private let onMenu: ref<CallbackHandle>;
    private let settingsListener: ref<VolumeSettingsListener>;

    private func OnAttach() -> Void {
        OnGameSystemAttach();
        this.onMenu = GameInstance.GetBlackboardSystem(this.GetGameInstance())
        .Get(GetAllBlackboardDefs().UI_System)
        .RegisterListenerBool(GetAllBlackboardDefs().UI_System.IsInMenu, this, n"OnMenu");

        this.settingsListener = new VolumeSettingsListener();
        this.settingsListener.Initialize(this.GetGameInstance());
        this.settingsListener.Start();
    }
    private func OnDetach() -> Void {
        OnGameSystemDetach();
        this.settingsListener = null;
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