module Audioware

private native func RegisterEmitter(id: EntityID) -> Void;
private native func UnregisterEmitter(id: EntityID) -> Void;
private native func UpdateActorLocation(id: EntityID, position: Vector4, orientation: Quaternion) -> Void;
private native func EmittersCount() -> Int32;

public class Audioware extends ScriptableSystem {
    private let m_callbackSystem: wref<CallbackSystem>;
    public let m_subtitleDelayID: DelayID;
    public let m_subtitleRemaining: Float = 0.0;
    public let m_subtitleLine: scnDialogLineData;
    public let m_positionDelayID: DelayID;
    public let m_positionsDelayID: DelayID;
    private let m_emitters: array<EntityID>;
    private let m_menuListener: ref<CallbackHandle>;
    private let m_deathListener: ref<CallbackHandle>;

    public func RegisterVentriloquist(id: EntityID) -> Void {
        LogChannel(n"DEBUG", s"register ventriloquist (\(EntityID.ToDebugString(id)))");
        RegisterEmitter(id);
        if Equals(this.m_positionsDelayID, GetInvalidDelayID()) {
            let callback = new UpdateEmitterCallback();
            callback.npc = GameInstance.FindEntityByID(this.GetGameInstance(), id) as NPCPuppet;
            this.m_positionsDelayID = GameInstance
            .GetDelaySystem(this.GetGameInstance())
            .DelayCallback(callback, 0.1, true);
        }
    }

    public func UnregisterVentriloquist(id: EntityID) -> Void {
        LogChannel(n"DEBUG", s"unregister ventriloquist (\(EntityID.ToDebugString(id)))");
        UnregisterEmitter(id);
        let size = EmittersCount();
        LogChannel(n"DEBUG", s"emitters count (\(ToString(size)))");
        if size == 0 && NotEquals(this.m_positionsDelayID, GetInvalidDelayID()) {
            GameInstance
            .GetDelaySystem(this.GetGameInstance())
            .CancelCallback(this.m_positionsDelayID);
            this.m_positionsDelayID = GetInvalidDelayID();
        }
    }

    private func OnAttach() {
        this.m_emitters = [];
        this.m_positionsDelayID = GetInvalidDelayID();
        this.m_callbackSystem = GameInstance.GetCallbackSystem();
        this.m_callbackSystem.RegisterCallback(n"Session/BeforeStart", this, n"OnSessionBeforeStart");
        this.m_callbackSystem.RegisterCallback(n"Session/Start", this, n"OnSessionStart");
        this.m_callbackSystem.RegisterCallback(n"Session/Ready", this, n"OnSessionReady");
        this.m_callbackSystem.RegisterCallback(n"Session/BeforeEnd", this, n"OnSessionBeforeEnd");
        this.m_callbackSystem.RegisterCallback(n"Entity/Uninitialize", this, n"OnEntityUninitialize");
        let ui: ref<IBlackboard> = GameInstance
        .GetBlackboardSystem(this.GetGameInstance())
        .Get(GetAllBlackboardDefs().UI_System);
        this.m_menuListener = ui.RegisterListenerBool(GetAllBlackboardDefs().UI_System.IsInMenu, this, n"OnInMenu");
        let psm: ref<IBlackboard> = GameInstance
        .GetBlackboardSystem(this.GetGameInstance())
        .Get(GetAllBlackboardDefs().PlayerStateMachine);
        this.m_deathListener = psm.RegisterListenerBool(GetAllBlackboardDefs().PlayerStateMachine.DisplayDeathMenu, this, n"OnDeathMenu");
    }

    private func OnDetach() {
        this.m_callbackSystem.UnregisterCallback(n"Session/BeforeStart", this, n"OnSessionBeforeStart");
        this.m_callbackSystem.UnregisterCallback(n"Session/Ready", this, n"OnSessionReady");
        this.m_callbackSystem.UnregisterCallback(n"Session/BeforeEnd", this, n"OnSessionBeforeEnd");
        this.m_callbackSystem = null;
        if NotEquals(this.m_positionsDelayID, GetInvalidDelayID()) {
            GameInstance
            .GetDelaySystem(this.GetGameInstance())
            .CancelCallback(this.m_positionsDelayID);
            this.m_positionsDelayID = GetInvalidDelayID();
        }
        let ui: ref<IBlackboard> = GameInstance
        .GetBlackboardSystem(this.GetGameInstance())
        .Get(GetAllBlackboardDefs().UI_System);
        ui.UnregisterListenerBool(GetAllBlackboardDefs().UI_System.IsInMenu, this.m_menuListener);
        let psm: ref<IBlackboard> = GameInstance
        .GetBlackboardSystem(this.GetGameInstance())
        .Get(GetAllBlackboardDefs().PlayerStateMachine);
        psm.UnregisterListenerBool(GetAllBlackboardDefs().PlayerStateMachine.DisplayDeathMenu, this.m_deathListener);
    }

    private final func OnPlayerAttach(request: ref<PlayerAttachRequest>) -> Void {
        let player = request.owner as PlayerPuppet;
        if IsDefined(player) {
            let callback = new UpdateListenerCallback();
            callback.player = player;
            this.m_positionDelayID = GameInstance
            .GetDelaySystem(this.GetGameInstance())
            .DelayCallback(callback, 0.1, true);
        }
    }

    private final func OnPlayerDetach(request: ref<PlayerDetachRequest>) -> Void {
        if NotEquals(GetInvalidDelayID(), this.m_positionDelayID) {
            GameInstance
            .GetDelaySystem(this.GetGameInstance())
            .CancelCallback(this.m_positionDelayID);
            this.m_positionDelayID = GetInvalidDelayID();
        }
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
    private cb func OnEntityUninitialize(event: ref<EntityLifecycleEvent>) {
        let id = event.GetEntity().GetEntityID();
        UnregisterEmitter(id);
    }
    protected cb func OnInMenu(value: Bool) -> Bool {
        LogChannel(n"DEBUG", s"in menu: \(ToString(value))");
        let state: EngineState = value ? EngineState.InMenu : EngineState.InGame;
        UpdateEngineState(state);
        return false;
    }
    protected cb func OnDeathMenu(value: Bool) -> Bool {
        LogChannel(n"DEBUG", s"death menu: \(ToString(value))");
        if value { UpdateEngineState(EngineState.InMenu); }
        return false;
    }

    public static final func GetInstance(game: GameInstance) -> ref<Audioware> {
        let container = GameInstance.GetScriptableSystemsContainer(game);
        return container.Get(n"Audioware.Audioware") as Audioware;
    } 
}

public class UpdateListenerCallback extends DelayCallback {
    public let player: wref<PlayerPuppet>;
    public func Call() -> Void {
        // LogChannel(n"DEBUG", "update listener callback");
        if IsDefined(this.player) {
            let system = Audioware.GetInstance(this.player.GetGame());
            let id = this.player.GetEntityID();
            let pos = this.player.GetWorldPosition();
            let orientation = this.player.GetWorldOrientation();
            UpdateActorLocation(id, pos, orientation);
            system.m_positionDelayID = GameInstance
            .GetDelaySystem(this.player.GetGame())
            .DelayCallback(this, 0.1, true);
            // Vector4.Distance
            // Vector4.Length
        }
    }
}

public class UpdateEmitterCallback extends DelayCallback {
  public let npc: wref<GameObject>;
  public func Call() -> Void {
    LogChannel(n"DEBUG", "update emitter callback");
    if IsDefined(this.npc) {
        let system = Audioware.GetInstance(this.npc.GetGame());
        let id = this.npc.GetEntityID();
        let pos = this.npc.GetWorldPosition();
        let orientation = this.npc.GetWorldOrientation();
        UpdateActorLocation(id, pos, orientation);
        system.m_positionsDelayID = GameInstance
        .GetDelaySystem(this.npc.GetGame())
        .DelayCallback(this, 0.1, true);
    }
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
    Audioware.GetInstance(game).m_subtitleRemaining = 0.0;
    Audioware.GetInstance(game).m_subtitleDelayID = GetInvalidDelayID();
    let board: ref<IBlackboard> = GameInstance.GetBlackboardSystem(game).Get(GetAllBlackboardDefs().UIGameData);
    board.SetVariant(GetAllBlackboardDefs().UIGameData.HideDialogLine, [this.line.id], true);
  }
}