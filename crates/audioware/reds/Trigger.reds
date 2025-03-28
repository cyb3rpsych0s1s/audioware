module Audioware

@wrapMethod(worldScriptedAudioSignpostTrigger)
protected cb func OnPlayerEnter(localPlayer: ref<GameObject>) -> Bool {
    FTLog(s"worldScriptedAudioSignpostTrigger.OnPlayerEnter");
    return wrappedMethod(localPlayer);
}

@wrapMethod(worldScriptedAudioSignpostTrigger)
protected cb func OnPlayerExit(localPlayer: ref<GameObject>) -> Bool {
    FTLog(s"worldScriptedAudioSignpostTrigger.OnPlayerExit");
    return wrappedMethod(localPlayer);
}