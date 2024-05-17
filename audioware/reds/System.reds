module Audioware

private native func RegisterEmitter(id: EntityID) -> Void;
private native func UnregisterEmitter(id: EntityID) -> Void;
private native func UpdateActorLocation(id: EntityID, position: Vector4, orientation: Quaternion) -> Void;
private native func EmittersCount() -> Int32;
private native func UpdatePlayerReverb(value: Float) -> Bool;
private native func UpdatePlayerPreset(preset: Preset) -> Bool;

public class Audioware extends ScriptableSystem {
    public let m_subtitleDelayID: DelayID;
    public let m_subtitleRemaining: Float = 0.0;
    public let m_subtitleLine: scnDialogLineData;
    public let m_positionDelayID: DelayID;
    public let m_positionsDelayID: DelayID;
    private let m_emitters: array<EntityID>;
    private let m_menuListener: ref<CallbackHandle>;
    private let m_deathListener: ref<CallbackHandle>;
    private let m_playerReverbListener: ref<CallbackHandle>;
    private let m_playerPresetListener: ref<CallbackHandle>;

    public func RegisterVentriloquist(id: EntityID) -> Void {
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
        UnregisterEmitter(id);
        let size = EmittersCount();
        if size == 0 && NotEquals(this.m_positionsDelayID, GetInvalidDelayID()) {
            GameInstance
            .GetDelaySystem(this.GetGameInstance())
            .CancelCallback(this.m_positionsDelayID);
            this.m_positionsDelayID = GetInvalidDelayID();
        }
    }

    private func OnAttach() {
        UpdateEngineState(EngineState.Start);
        this.m_positionsDelayID = GetInvalidDelayID();
        let ui: ref<IBlackboard> = GameInstance
        .GetBlackboardSystem(this.GetGameInstance())
        .Get(GetAllBlackboardDefs().UI_System);
        this.m_menuListener = ui.RegisterListenerBool(GetAllBlackboardDefs().UI_System.IsInMenu, this, n"OnInMenu");
        let psm: ref<IBlackboard> = GameInstance
        .GetBlackboardSystem(this.GetGameInstance())
        .Get(GetAllBlackboardDefs().PlayerStateMachine);
        this.m_deathListener = psm.RegisterListenerBool(GetAllBlackboardDefs().PlayerStateMachine.DisplayDeathMenu, this, n"OnDeathMenu");
        this.ResetPreset();
        this.ResetReverb();
        GameInstance.GetCallbackSystem()
        .RegisterCallback(n"Session/BeforeEnd", this, n"OnSessionBeforeEnd").SetRunMode(CallbackRunMode.Once);
        GameInstance.GetCallbackSystem().RegisterCallback(n"Entity/Uninitialize", this, n"OnEntityUninitialize");
    }

    private func OnDetach() {
        UpdateEngineState(EngineState.End);
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
        let boards: ref<BlackboardSystem>;
        let board: ref<IBlackboard>;
        let defs = GetAllBlackboardDefs();
        let player = request.owner as PlayerPuppet;
        boards = GameInstance.GetBlackboardSystem(this.GetGameInstance());
        board = boards.Get(defs.AudiowareSettings);
        this.m_playerReverbListener = board.RegisterListenerFloat(defs.AudiowareSettings.PlayerReverb, this, n"OnReverbChanged", false);
        this.m_playerPresetListener = board.RegisterListenerInt(defs.AudiowareSettings.PlayerPreset, this, n"OnPlayerPresetChanged", false);
        if IsDefined(player) {
            UpdateEngineState(EngineState.InGame);
            let callback = new UpdateListenerCallback();
            callback.player = player;
            this.m_positionDelayID = GameInstance
            .GetDelaySystem(this.GetGameInstance())
            .DelayCallback(callback, 0.1, true);
        }
    }

    private final func OnPlayerDetach(request: ref<PlayerDetachRequest>) -> Void {
        let boards: ref<BlackboardSystem>;
        let board: ref<IBlackboard>;
        let defs = GetAllBlackboardDefs();
        boards = GameInstance.GetBlackboardSystem(this.GetGameInstance());
        board = boards.Get(defs.AudiowareSettings);
        board.UnregisterListenerFloat(defs.AudiowareSettings.PlayerReverb, this.m_playerReverbListener);
        board.UnregisterListenerInt(defs.AudiowareSettings.PlayerPreset, this.m_playerPresetListener);
        this.m_playerReverbListener = null;
        this.m_playerPresetListener = null;
        if NotEquals(GetInvalidDelayID(), this.m_positionDelayID) {
            GameInstance
            .GetDelaySystem(this.GetGameInstance())
            .CancelCallback(this.m_positionDelayID);
            this.m_positionDelayID = GetInvalidDelayID();
        }
    }
    
    private cb func OnSessionBeforeEnd(event: ref<GameSessionEvent>) {
        GameInstance.GetCallbackSystem().UnregisterCallback(n"Entity/Uninitialize", this, n"OnEntityUninitialize");
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
    protected cb func OnReverbChanged(value: Float) -> Bool {
        let result = UpdatePlayerReverb(value);
        return result;
    }
    protected cb func OnPlayerPresetChanged(value: Int32) -> Bool {
        let preset: Preset = IntEnum<Preset>(value);
        let result = UpdatePlayerPreset(preset);
        return result;
    }
    public func ResetReverb() -> Bool {
        let reset = UpdatePlayerReverb(0.);
        return reset;
    }
    public func ResetPreset() -> Bool {
        let reset = UpdatePlayerPreset(Preset.None);
        return reset;
    }

    public static final func GetInstance(game: GameInstance) -> ref<Audioware> {
        let container = GameInstance.GetScriptableSystemsContainer(game);
        return container.Get(n"Audioware.Audioware") as Audioware;
    }
}

public class UpdateListenerCallback extends DelayCallback {
    public let player: wref<PlayerPuppet>;
    public func Call() -> Void {
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