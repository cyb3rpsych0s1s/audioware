module Audioware

@wrapMethod(gameuiInGameMenuGameController)
protected cb func OnDeathScreenDelayEvent(evt: ref<DeathMenuDelayEvent>) -> Bool {
    LOG(s"on display death menu");
    Shutdown();
    return wrappedMethod(evt);
}

@wrapMethod(GameObject)
protected cb func OnWillDieSoonEventEvent(evt: ref<WillDieSoonEvent>) -> Bool {
    LOG(s"on will die soon: \(EntityID.ToDebugString(this.GetEntityID()))");
    UnregisterEmitter(this.GetEntityID());
    wrappedMethod(evt);
}

@wrapMethod(ScriptedPuppet)
protected cb func OnDeath(evt: ref<gameDeathEvent>) -> Bool {
    LOG(s"on death: \(EntityID.ToDebugString(this.GetEntityID()))");
    UnregisterEmitter(this.GetEntityID());
    return wrappedMethod(evt);
}

@wrapMethod(VehicleComponent)
protected cb func OnDeath(evt: ref<gameDeathEvent>) -> Bool {
    LOG(s"on death: \(EntityID.ToDebugString(this.GetEntity().GetEntityID()))");
    UnregisterEmitter(this.GetEntity().GetEntityID());
    return wrappedMethod(evt);
}

@wrapMethod(AIHumanComponent)
protected cb func OnDeath(evt: ref<gameDeathEvent>) -> Bool {
    LOG(s"on death: \(EntityID.ToDebugString(this.GetEntity().GetEntityID()))");
    UnregisterEmitter(this.GetEntity().GetEntityID());
    return wrappedMethod(evt);
}

@wrapMethod(Device)
protected cb func OnDeath(evt: ref<gameDeathEvent>) -> Bool {
    LOG(s"on death: \(EntityID.ToDebugString(this.GetEntityID()))");
    UnregisterEmitter(this.GetEntityID());
    return wrappedMethod(evt);
}
