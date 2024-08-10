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
    LOG(s"on will die soon: \(EntityID.ToDebugString(this.GetEntityID())) [GameObject]");
    if !this.IsPlayer() {
        GameInstance.GetAudioSystemExt(GetGameInstance()).OnEmitterDies(this.GetEntityID());
    }
    wrappedMethod(evt);
}

/// whenever any vehicle is destroyed
@wrapMethod(VehicleComponent)
protected cb func OnDeath(evt: ref<gameDeathEvent>) -> Bool {
    LOG(s"on death: \(EntityID.ToDebugString(this.GetEntity().GetEntityID())) [VehicleComponent]");
    GameInstance.GetAudioSystemExt(GetGameInstance()).OnEmitterDies(this.GetEntity().GetEntityID());
    return wrappedMethod(evt);
}

/// whenever any AI human dies
@wrapMethod(AIHumanComponent)
protected cb func OnDeath(evt: ref<gameDeathEvent>) -> Bool {
    LOG(s"on death: \(EntityID.ToDebugString(this.GetEntity().GetEntityID())) [AIHumanComponent]");
    GameInstance.GetAudioSystemExt(GetGameInstance()).OnEmitterDies(this.GetEntity().GetEntityID());
    return wrappedMethod(evt);
}

/// whenever any device is destroyed
@wrapMethod(Device)
protected cb func OnDeath(evt: ref<gameDeathEvent>) -> Bool {
    LOG(s"on death: \(EntityID.ToDebugString(this.GetEntityID())) [Device]");
    GameInstance.GetAudioSystemExt(GetGameInstance()).OnEmitterDies(this.GetEntityID());
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
