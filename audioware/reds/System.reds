module Audioware

public class AudiowareSystem extends ScriptableSystem {
    private let onMenu: ref<CallbackHandle>;
    private func OnAttach() -> Void {
        OnGameSystemAttach();
        this.onMenu = GameInstance.GetBlackboardSystem(this.GetGameInstance())
        .Get(GetAllBlackboardDefs().UI_System)
        .RegisterListenerBool(GetAllBlackboardDefs().UI_System.IsInMenu, this, n"OnMenu");
    }
    private func OnDetach() -> Void {
        OnGameSystemDetach();
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