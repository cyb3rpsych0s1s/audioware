module Audioware

public native class gameAudioSystemExt extends ScriptableSystem {
    // enhanced SDK
    public final native func Play(eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt line: scnDialogLineType, opt ext: ref<AudioSettingsExt>) -> Void;
    public final native func Stop(eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt tween: ref<Tween>) -> Void
    public final native func Switch(switchName: CName, switchValue: CName, opt entityID: EntityID, opt emitterName: CName, opt switchNameTween: ref<Tween>, opt switchValueSettings: ref<AudioSettingsExt>) -> Void;
    public final native func PlayOverThePhone(eventName: CName, emitterName: CName, gender: CName) -> Void;
    // enhanced SDK variants
    public final func Play(eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt line: scnDialogLineType, opt tween: ref<Tween>) -> Void {
        this.Play(eventName, entityID, emitterName, line, AudioSettingsExtBuilder.Create().WithFadeInTween(tween).Build());
    }
    public final func Switch(switchName: CName, switchValue: CName, opt entityID: EntityID, opt emitterName: CName, opt switchNameTween: ref<Tween>, opt switchValueTween: ref<Tween>) -> Void {
        this.Switch(switchName, switchValue, entityID, emitterName, switchNameTween, AudioSettingsExtBuilder.Create().WithFadeInTween(switchValueTween).Build()); 
    }
        
    // spatial scene
    public final native func IsRegisteredEmitter(entityID: EntityID) -> Bool;
    public final native func EmittersCount() -> Int32;
    public final native func PlayOnEmitter(eventName: CName, entityID: EntityID, emitterName: CName, opt tween: ref<Tween>) -> Void;
    public final native func StopOnEmitter(eventName: CName, entityID: EntityID, emitterName: CName, opt tween: ref<Tween>) -> Void;
    public final native func OnEmitterDies(entityID: EntityID) -> Void;
}
