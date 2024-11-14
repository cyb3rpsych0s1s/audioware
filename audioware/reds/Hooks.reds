module Audioware

/// whenever V dies, after animation where (s)he falls on the ground
@wrapMethod(gameuiInGameMenuGameController)
protected cb func OnDeathScreenDelayEvent(evt: ref<DeathMenuDelayEvent>) -> Bool {
    return wrappedMethod(evt);
}

/// whenever any vehicle is destroyed
@wrapMethod(VehicleComponent)
protected cb func OnDeath(evt: ref<gameDeathEvent>) -> Bool {
    GameInstance.GetAudioSystemExt(this.GetVehicle().GetGame()).OnEmitterDies(this.GetEntity().GetEntityID());
    return wrappedMethod(evt);
}

/// whenever any AI human dies
@wrapMethod(AIHumanComponent)
protected cb func OnDeath(evt: ref<gameDeathEvent>) -> Bool {
    GameInstance.GetAudioSystemExt(this.GetGame()).OnEmitterDies(this.GetEntity().GetEntityID());
    return wrappedMethod(evt);
}

/// whenever any device is destroyed
@wrapMethod(Device)
protected cb func OnDeath(evt: ref<gameDeathEvent>) -> Bool {
    GameInstance.GetAudioSystemExt(this.GetGame()).OnEmitterDies(this.GetEntityID());
    return wrappedMethod(evt);
}
