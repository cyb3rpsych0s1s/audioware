module Audioware

public class Audioware extends ScriptableSystem {
    private let m_callbackSystem: wref<CallbackSystem>;
    public let m_subtitleDelayID: DelayID;

    private func OnAttach() {
        this.m_callbackSystem = GameInstance.GetCallbackSystem();
        this.m_callbackSystem.RegisterCallback(n"Session/BeforeStart", this, n"OnSessionBeforeStart");
        this.m_callbackSystem.RegisterCallback(n"Session/Start", this, n"OnSessionStart");
        this.m_callbackSystem.RegisterCallback(n"Session/Ready", this, n"OnSessionReady");
        this.m_callbackSystem.RegisterCallback(n"Session/BeforeEnd", this, n"OnSessionBeforeEnd");
    }

    private func OnDetach() {
        this.m_callbackSystem.UnregisterCallback(n"Session/BeforeStart", this, n"OnSessionBeforeStart");
        this.m_callbackSystem.UnregisterCallback(n"Session/Ready", this, n"OnSessionReady");
        this.m_callbackSystem.UnregisterCallback(n"Session/BeforeEnd", this, n"OnSessionBeforeEnd");
        this.m_callbackSystem = null;
    }

    private cb func OnSessionBeforeStart(event: ref<GameSessionEvent>) {
        UpdateEngineState(EngineState.Start);
    }
    private cb func OnSessionReady(event: ref<GameSessionEvent>) {
        UpdateEngineState(EngineState.InGame);
    }
    private cb func OnSessionBeforeEnd(event: ref<GameSessionEvent>) {
        UpdateEngineState(EngineState.End);
    }

    public static final func GetInstance(game: GameInstance) -> ref<Audioware> {
        let container = GameInstance.GetScriptableSystemsContainer(game);
        return container.Get(n"Audioware.Audioware") as Audioware;
    } 
}

public class HideSubtitleCallback extends DelayCallback {
  private let line: scnDialogLineData;
  public func Call() -> Void {
    if !IsDefined(this.line.speaker) { return; }
    let game = this.line.speaker.GetGame();
    GameInstance
    .GetDelaySystem(game)
    .CancelCallback(Audioware.GetInstance(game).m_subtitleDelayID);
    let board: ref<IBlackboard> = GameInstance.GetBlackboardSystem(game).Get(GetAllBlackboardDefs().UIGameData);
    board.SetVariant(GetAllBlackboardDefs().UIGameData.HideDialogLine, [this.line.id], true);
  }
}