module Audioware

@wrapMethod(gameuiInGameMenuGameController)
protected cb func OnDeathScreenDelayEvent(evt: ref<DeathMenuDelayEvent>) -> Bool {
    LOG(s"on display death menu");
    Shutdown();
    return wrappedMethod(evt);
}