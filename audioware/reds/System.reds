module Audioware

private native func StopEngine() -> Void;

public class Audioware extends ScriptableSystem {

    private func OnAttach() {
        UpdateGameState(EngineState.Start);
    }

    private func OnDetach() {
        UpdateGameState(EngineState.End);
        StopEngine();
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
