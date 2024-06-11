module Audioware

@wrapMethod(LocomotionSwimming)
protected final const func GetCurrentDepth(const stateContext: ref<StateContext>, const scriptInterface: ref<StateGameScriptInterface>) -> Float {
    let currentDepth: Float = wrappedMethod(stateContext, scriptInterface);
    GameInstance.GetAudioSystem(scriptInterface.executionOwner.GetGame())
    .GlobalParameter(n"audioware_frequencies", currentDepth);
    return currentDepth;
}