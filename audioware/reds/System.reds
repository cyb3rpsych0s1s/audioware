module Audioware

public native func TestPlay() -> Void;

public class AudiowareSystem extends ScriptableSystem {
    private let menuListener: ref<CallbackHandle>;

    private func OnAttach() -> Void {
        FTLog(AsRef("on attach: AudiowareSystem"));
        SetGameState(GameState.Start);
        let system: ref<BlackboardSystem> = GameInstance.GetBlackboardSystem(this.GetGameInstance());
        let definitions: ref<AllBlackboardDefinitions> = GetAllBlackboardDefs();
        let ui: ref<IBlackboard> = system.Get(definitions.UI_System);
        this.menuListener = ui.RegisterListenerBool(definitions.UI_System.IsInMenu, this, n"OnInMenu");
    }
    private func OnDetach() -> Void {
        FTLog(AsRef("on detach: AudiowareSystem"));
        SetGameState(GameState.End);
        let system: ref<BlackboardSystem> = GameInstance.GetBlackboardSystem(this.GetGameInstance());
        let definitions: ref<AllBlackboardDefinitions> = GetAllBlackboardDefs();
        if IsDefined(this.menuListener) {
            let ui: ref<IBlackboard> = system.Get(definitions.UI_System);
            ui.UnregisterListenerBool(definitions.UI_System.IsInMenu, this.menuListener);
            this.menuListener = null;
        }
    }
    private final func OnPlayerAttach(request: ref<PlayerAttachRequest>) -> Void {
        FTLog(AsRef("on player attach: AudiowareSystem"));
        SetGameState(GameState.InGame);
        TestPlay();
    }
    private final func OnPlayerDetach(request: ref<PlayerDetachRequest>) -> Void {
        FTLog(AsRef("on player detach: AudiowareSystem"));
        UnsetPlayerGender();
    }
    protected cb func OnInMenu(value: Bool) -> Bool {
        SetGameState(value ? GameState.InMenu : GameState.InGame);
    }
}
