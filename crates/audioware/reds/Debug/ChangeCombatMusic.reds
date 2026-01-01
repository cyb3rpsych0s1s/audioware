import Audioware.*

public func NoAudioDilation() -> ref<AudioSettingsExt> {
    let ext = new AudioSettingsExt();
    ext.affectedByTimeDilation = false;
    return ext;
}

public class ChangeCombatMusic extends ScriptableSystem {
    public let wanted: ref<CallbackHandle>;
    private let dynamic: ref<DynamicSoundEvent>;
    private let volume: Float = 1.;
    private let playing: CName = n"None";
    private final func OnPlayerAttach(request: ref<PlayerAttachRequest>) -> Void {
        let manager = new AudioEventManager();
        // mute vanilla combat music
        manager.Mute(n"gmp_ui_prevention_player_commit_crime");
    }
    public func OnHeatChanged(heat: EPreventionHeatStage, agressiveness: Float) {
        let tween: ref<Tween>;
        let volume = 0.9 + (0.05 * Cast<Float>(EnumInt(heat)));
        FTLog(s"ChangeCombatMusic.OnHeatChanged( heat: \(ToString(heat)), agressiveness: \(agressiveness) )");
        // when chase starts
        if !IsDefined(this.dynamic)
        && EnumInt(heat) > 1 {
            // enqueue and play song
            this.dynamic = DynamicSoundEvent.Create(n"faf_la_cavale_instru", NoAudioDilation());
            let v = GetPlayer(this.GetGameInstance());
            v.QueueEvent(this.dynamic);
        }
        // when chase intensifies
        if IsDefined(this.dynamic)
        && EnumInt(heat) > 2 {
            this.dynamic.Stop();
            let ext = new AudioSettingsExt();
            ext.affectedByTimeDilation = false;
            // enqueue and play song
            this.dynamic = DynamicSoundEvent.Create(n"faf_la_cavale_instru_scratch", NoAudioDilation());
            let v = GetPlayer(this.GetGameInstance());
            v.QueueEvent(this.dynamic);
        }
        // when chase ends
        if IsDefined(this.dynamic)
        && EnumInt(heat) == 0 {
            tween = LinearTween.Immediate(2.);
            this.dynamic.Stop(tween);
        }
        // change volume based on heat stage
        if IsDefined(this.dynamic)
        && EnumInt(heat) > 0
        && volume != this.volume {
            this.volume = volume;
            tween = ElasticTween.ImmediateOut(5. / Cast<Float>(EnumInt(heat)), Cast<Float>(EnumInt(heat)) / 2.);
            this.dynamic.SetVolume(volume, tween);
        }
    }
    public func OnViewerChanged(inSight: Bool) {
        if IsDefined(this.dynamic) {
            FTLog(s"ChangeCombatMusic.OnViewerChanged( inSight: \(inSight) )");
            let tween: ref<Tween> = LinearTween.Immediate(3.);
            if inSight {
                this.dynamic.SetVolume(this.volume, tween);
            } else {
                // slightly increase volume when out of sight
                this.dynamic.SetVolume(this.volume + 0.05, tween);
            }
        }
    }
    public func OnLocomotionChanged(state: Int32) {
        let sprinting = state == 2;
        let crouching = state == 1;
        if IsDefined(this.dynamic) {
            FTLog(s"ChangeCombatMusic.OnLocomotionChanged( state: \(state) )");
            let tween = new LinearTween();
            tween.startTime = .5;
            tween.duration = 1.;
            if crouching {
                this.dynamic.SetVolume(this.volume - 0.1, tween);
            } else if sprinting {
                this.dynamic.SetVolume(this.volume + 0.1, tween);
            }
        }
    }
    public func OnDeath() {
        if IsDefined(this.dynamic) {
            let tween = LinearTween.Immediate(1.5);
            this.dynamic.Stop(tween);
        }
    }
    private func OnDetach() -> Void {
        if IsDefined(this.dynamic) {
            this.dynamic = null;
        }
    }
    public final static func GetInstance(game: GameInstance) -> ref<ChangeCombatMusic> = GameInstance
        .GetScriptableSystemsContainer(game)
        .Get(n"ChangeCombatMusic") as ChangeCombatMusic;
}

// notify heat stage changes
@wrapMethod(PreventionSystem)
private final func OnHeatChanged(previousHeat: EPreventionHeatStage) -> Void {
    wrappedMethod(previousHeat);
    if NotEquals(previousHeat, this.m_heatStage) {
        ChangeCombatMusic
            .GetInstance(this.GetGame())
            .OnHeatChanged(this.m_heatStage, this.m_chaseMultiplier);
    }
}

// notify how many agents can or cannot see player
@wrapMethod(PreventionSystem)
private final func HasViewersChanged(currentViewerState: Bool) -> Void {
    if NotEquals(currentViewerState, this.m_hasViewers) {
        ChangeCombatMusic
            .GetInstance(this.GetGame())
            .OnViewerChanged(this.m_hasViewers);
    }
}

// notify when player starts sprinting or crouching
@wrapMethod(PlayerPuppet)
protected cb func OnLocomotionStateChanged(newState: Int32) -> Bool {
    let previous = this.m_locomotionState;

    let out = wrappedMethod(newState);

    if NotEquals(previous, newState) {
        ChangeCombatMusic
            .GetInstance(this.GetGame())
            .OnLocomotionChanged(newState);
    }

    return out;
}

// notify when player is about to die
@wrapMethod(PlayerPuppet)
protected cb func OnDeath(evt: ref<gameDeathEvent>) -> Bool {
    ChangeCombatMusic
        .GetInstance(this.GetGame())
        .OnDeath();
    return wrappedMethod(evt);
}
