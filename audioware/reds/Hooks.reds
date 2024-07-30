module Audioware

/// whenever V dies, after animation where (s)he falls on the ground
@wrapMethod(gameuiInGameMenuGameController)
protected cb func OnDeathScreenDelayEvent(evt: ref<DeathMenuDelayEvent>) -> Bool {
    LOG(s"on display death menu");
    Shutdown();
    return wrappedMethod(evt);
}

/// whenever any NPC is about to die
@wrapMethod(GameObject)
protected cb func OnWillDieSoonEventEvent(evt: ref<WillDieSoonEvent>) -> Bool {
    LOG(s"on will die soon: \(EntityID.ToDebugString(this.GetEntityID()))");
    UnregisterEmitter(this.GetEntityID());
    wrappedMethod(evt);
}

/// whenever any NPC dies
@wrapMethod(ScriptedPuppet)
protected cb func OnDeath(evt: ref<gameDeathEvent>) -> Bool {
    LOG(s"on death: \(EntityID.ToDebugString(this.GetEntityID()))");
    UnregisterEmitter(this.GetEntityID());
    return wrappedMethod(evt);
}

/// whenever any vehicle is destroyed
@wrapMethod(VehicleComponent)
protected cb func OnDeath(evt: ref<gameDeathEvent>) -> Bool {
    LOG(s"on death: \(EntityID.ToDebugString(this.GetEntity().GetEntityID()))");
    UnregisterEmitter(this.GetEntity().GetEntityID());
    return wrappedMethod(evt);
}

/// whenever any AI human dies
@wrapMethod(AIHumanComponent)
protected cb func OnDeath(evt: ref<gameDeathEvent>) -> Bool {
    LOG(s"on death: \(EntityID.ToDebugString(this.GetEntity().GetEntityID()))");
    UnregisterEmitter(this.GetEntity().GetEntityID());
    return wrappedMethod(evt);
}

/// whenever any device is destroyed
@wrapMethod(Device)
protected cb func OnDeath(evt: ref<gameDeathEvent>) -> Bool {
    LOG(s"on death: \(EntityID.ToDebugString(this.GetEntityID()))");
    UnregisterEmitter(this.GetEntityID());
    return wrappedMethod(evt);
}

/// whenever quitting game from pause menu
@wrapMethod(PauseMenuGameController)
protected cb func OnMenuItemActivated(index: Int32, target: ref<ListItemController>) -> Bool {
    let data: ref<PauseMenuListItemData>;
    data = target.GetData() as PauseMenuListItemData;
    switch data.action {
        case PauseMenuAction.OpenSubMenu:
        case PauseMenuAction.Save:
        case PauseMenuAction.QuickSave:
            break;
        case PauseMenuAction.ExitGame:
        case PauseMenuAction.ExitToMainMenu:
            Shutdown();
            break;
    }
    return wrappedMethod(index, target);
}

public class VolumeSettingsListener extends ConfigVarListener {
    private let game: GameInstance;

    public func Initialize(game: GameInstance) {
        LOG("initialize VolumeSettingsListener");
        this.game = game;
    }

    public func Start() {
        this.Register(n"/audio/volume");
    }

    protected cb func OnVarModified(groupPath: CName, varName: CName, varType: ConfigVarType, reason: ConfigChangeReason) {
        LOG(s"groupPath: \(NameToString(groupPath)), varName: \(NameToString(varName)), varType: \(ToString(varType)), reason: \(ToString(reason))");
        if Equals(groupPath, n"/audio/volume") && Equals(reason, ConfigChangeReason.Accepted) {
            switch varName {
                case n"MasterVolume":
                case n"SfxVolume":
                case n"DialogueVolume":
                case n"MusicVolume":
                case n"CarRadioVolume":
                case n"RadioportVolume":
                    let settings = GameInstance.GetSettingsSystem(this.game);
                    let setting = (settings.GetGroup(groupPath).GetVar(varName) as ConfigVarInt).GetValue();
                    SetVolume(varName, setting);
                    break;
                default:
                    break;
            }
        }
    }
}
