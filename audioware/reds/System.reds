module Audioware

public native func TestPlay() -> Void;

public class AudiowareSystem extends ScriptableSystem {
    private let attached: ref<CallbackSystemHandler>;
    private let detachedOnce: ref<CallbackSystemHandler>;
    private let attachedOnce: ref<CallbackSystemHandler>;

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
        .RegisterListenerFloat(definitions.Audioware_Settings.ReverbMix, this, n"OnPlayerReverb", false);
        this.playerPresetListener = audio
        .RegisterListenerInt(definitions.Audioware_Settings.AudioPreset, this, n"OnPlayerPreset", false);

        this.settingsListener = new VolumeSettingsListener();
        this.settingsListener.Initialize(this.GetGameInstance());
        this.settingsListener.Start();
        
        // spatial scene

        // use only with EntityID
        this.detachedOnce = GameInstance.GetCallbackSystem()
            .RegisterCallback(n"Entity/Detach", this, n"OnDespawn")
            .AddTarget(EntityTarget.Type(n"PlayerPuppet"))
            .SetRunMode(CallbackRunMode.OncePerTarget);

        // used for types and record IDs
        this.attached = GameInstance.GetCallbackSystem()
            .RegisterCallback(n"Entity/Attached", this, n"OnSpawn")
            // note: not specifying a target will trigger callback for each possible entity in-game
            .AddTarget(EntityTarget.Type(n"PlayerPuppet"));

        // use only with EntityID
        this.attachedOnce = GameInstance.GetCallbackSystem()
            .RegisterCallback(n"Entity/Attached", this, n"OnSpawn")
            .AddTarget(EntityTarget.Type(n"PlayerPuppet"))
            .SetRunMode(CallbackRunMode.OncePerTarget);
    }
    private func OnDetach() -> Void {
        LOG("on detach: AudiowareSystem");
        this.attached.Unregister();
        this.detachedOnce.Unregister();
        this.attached = null;
        this.detachedOnce = null;
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
            audio.UnregisterListenerFloat(definitions.Audioware_Settings.ReverbMix, this.playerReverbListener);
        }
        if IsDefined(this.playerPresetListener) {
            audio.UnregisterListenerInt(definitions.Audioware_Settings.AudioPreset, this.playerPresetListener);
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

    public func IsValidEmitter(recordID: TweakDBID) -> Bool {
        let record = TweakDBInterface.GetRecord(recordID);
        if !IsDefined(record) {
            return false;
        }
        let id = record.GetID();
        let invalid = [
            t"Character.Player_Puppet_Base",
            t"Character.Player_Puppet_Inventory",
            t"Character.Player_Puppet_Menu",
            t"Character.Player_Puppet_Photomode",
            t"Character.Player_Replacer_Puppet_Base"
        ];
        return !ArrayContains(invalid, id);
    }

    public func IsValidEmitter(entityID: EntityID) -> Bool {
        let game = GetGameInstance();
        let entity = GameInstance.FindEntityByID(game, entityID);
        if !IsDefined(entity) {
            return false;
        }
        return NotEquals(entityID, GetPlayer(game).GetEntityID());
    }

    public func IsValidEmitter(className: CName) -> Bool {
        return NotEquals(className, n"PlayerPuppet")
        && Reflection.GetClass(className).IsA(n"gameObject");
    }

    public func RegisterEmitter(entityID: EntityID, opt emitterName: CName, opt emitterSettings: EmitterSettings) -> Registration {
        let display = EntityID.ToDebugString(entityID);

        if !this.IsValidEmitter(entityID) {
            WARN(s"invalid emitter entity ID (\(display))");
            return Registration.Failed;
        }

        // if already spawned
        let entity = GameInstance.FindEntityByID(GetGameInstance(), entityID);
        if IsDefined(entity) {
            let registered = RegisterEmitter(entityID, emitterName, emitterSettings);
            if !registered {
                ERR(s"failed to register emitter entity ID (\(display))");
                return Registration.Failed;
            }
            this.detachedOnce.AddTarget(EntityTarget.ID(entityID));
            return Registration.Ready;
        }

        // otherwise
        this.attachedOnce.AddTarget(EntityTarget.ID(entityID));
        return Registration.Postponed;
    }

    public func UnregisterEmitter(entityID: EntityID) -> Void {
        if IsRegisteredEmitter(entityID) {
            UnregisterEmitter(entityID);
            this.detachedOnce.RemoveTarget(EntityTarget.ID(entityID));
        }
    }

    private cb func OnSpawn(event: ref<EntityLifecycleEvent>) {
        let entity = event.GetEntity();
        if !IsDefined(entity) { return; }
        let id = entity.GetEntityID();
        let display = EntityID.ToDebugString(id);
        if !entity.IsA(n"PlayerPuppet") {
            LOG(s"on emitter spawn: AudiowareSystem (\(display))");
        }
        // ignore EntityTarget placeholder, we only care about emitters here
        if !entity.IsA(n"PlayerPuppet") {
            let already = IsRegisteredEmitter(id);
            if !already {
                let registered = RegisterEmitter(id);
                if registered {
                    this.detachedOnce.AddTarget(EntityTarget.ID(id));
                    LOG(s"on emitter registered: AudiowareSystem (\(display))");
                }
            }
        }
    }

    private cb func OnDespawn(event: ref<EntityLifecycleEvent>) {
        let entity = event.GetEntity();
        if !IsDefined(entity) { return; }
        if !entity.IsA(n"PlayerPuppet") {
            let id = entity.GetEntityID();
            let display = EntityID.ToDebugString(id);
            LOG(s"on emitter despawn: AudiowareSystem (\(display))");
            let registered = IsRegisteredEmitter(id);
            if registered {
                LOG(s"on emitter despawn while registered: AudiowareSystem (\(display))");
                let unregistered = UnregisterEmitter(id);
                LOG(s"on emitter despawn + unregistered?[\(ToString(unregistered))]: AudiowareSystem (\(display))");
            }
        } else { LOG("on player despawn: AudiowareSystem"); }
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
        LOG(s"on reverb mix changed (\(ToString(value))): AudiowareSystem");
        SetReverbMix(value);
    }
    protected cb func OnPlayerPreset(value: Int32) -> Bool {
        let preset = IntEnum<Preset>(value);
        LOG(s"on player preset changed (\(ToString(preset))): AudiowareSystem");
        SetPreset(preset);
    }
    protected cb func OnSwim(value: Int32) -> Bool {
        let state = IntEnum<gamePSMSwimming>(value);
        let diving = Equals(state, gamePSMSwimming.Diving);
        SetPreset(diving ? Preset.Underwater : Preset.None);
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
