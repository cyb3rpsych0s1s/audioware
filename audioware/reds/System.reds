module Audioware

public class Audioware extends ScriptableSystem {
    private let menuListener: ref<CallbackHandle>;

    protected cb func OnInMenu(value: Bool) -> Bool {
        UpdateGameState(value ? EngineState.InMenu : EngineState.InGame);
    }

    private func OnAttach() {
        UpdateGameState(EngineState.Start);
        let system: ref<BlackboardSystem> = GameInstance.GetBlackboardSystem(this.GetGameInstance());
        let definitions: ref<AllBlackboardDefinitions> = GetAllBlackboardDefs();
        let ui: ref<IBlackboard> = system.Get(definitions.UI_System);
        this.menuListener = ui.RegisterListenerBool(definitions.UI_System.IsInMenu, this, n"OnInMenu");
    }

    private func OnDetach() {
        UpdateGameState(EngineState.End);
        StopEngine();

        let system: ref<BlackboardSystem> = GameInstance.GetBlackboardSystem(this.GetGameInstance());
        let definitions: ref<AllBlackboardDefinitions> = GetAllBlackboardDefs();
        if IsDefined(this.menuListener) {
            let ui: ref<IBlackboard> = system.Get(definitions.UI_System);
            ui.UnregisterListenerBool(definitions.UI_System.IsInMenu, this.menuListener);
            this.menuListener = null;
        }
    }

    private final func OnPlayerAttach(request: ref<PlayerAttachRequest>) -> Void {
        let player = request.owner as PlayerPuppet;
        if IsDefined(player) {
            UpdateGameState(EngineState.InGame);
        }
    }

    private final func OnPlayerDetach(request: ref<PlayerDetachRequest>) -> Void {}
    
    public static final func GetInstance(game: GameInstance) -> ref<Audioware> {
        let container = GameInstance.GetScriptableSystemsContainer(game);
        return container.Get(n"Audioware.Audioware") as Audioware;
    }
}
