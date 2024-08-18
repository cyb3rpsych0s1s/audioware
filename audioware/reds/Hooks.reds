module Audioware

/// whenever V dies, after animation where (s)he falls on the ground
@wrapMethod(gameuiInGameMenuGameController)
protected cb func OnDeathScreenDelayEvent(evt: ref<DeathMenuDelayEvent>) -> Bool {
    DBG(s"on display death menu");
    Shutdown();
    return wrappedMethod(evt);
}

/// whenever any vehicle is destroyed
@wrapMethod(VehicleComponent)
protected cb func OnDeath(evt: ref<gameDeathEvent>) -> Bool {
    let id = this.GetEntity().GetEntityID();
    let ext = GameInstance.GetAudioSystemExt(GetGameInstance());
    DBG(s"on death: \(EntityID.ToDebugString(id)) [VehicleComponent]");
    if ext.IsRegisteredEmitter(id) {
        ext.OnEmitterDies(id);
    }
    return wrappedMethod(evt);
}

/// whenever any AI human dies
@wrapMethod(AIHumanComponent)
protected cb func OnDeath(evt: ref<gameDeathEvent>) -> Bool {
    let id = this.GetEntity().GetEntityID();
    let ext = GameInstance.GetAudioSystemExt(this.GetGame());
    DBG(s"on death: \(EntityID.ToDebugString(id)) [AIHumanComponent]");
    if ext.IsRegisteredEmitter(id) {
        ext.OnEmitterDies(id);
    }
    return wrappedMethod(evt);
}

/// whenever any device is destroyed
@wrapMethod(Device)
protected cb func OnDeath(evt: ref<gameDeathEvent>) -> Bool {
    let id = this.GetEntityID();
    let ext = GameInstance.GetAudioSystemExt(this.GetGame());
    DBG(s"on death: \(EntityID.ToDebugString(id)) [Device]");
    if ext.IsRegisteredEmitter(id) {
        ext.OnEmitterDies(id);
    }
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
