import Audioware.*

public class DynamicSoundEvents extends ScriptableSystem {
    private let dynamic: ref<DynamicSoundEvent>;
    private let volume: Float = 1.;
    private final func OnPlayerAttach(request: ref<PlayerAttachRequest>) -> Void {
        let v = request.owner as PlayerPuppet;
        if IsDefined(v) 
            && !IsDefined(this.dynamic)
            && !GameInstance.GetSystemRequestsHandler().IsPreGame() {
            this.dynamic = DynamicSoundEvent.Create(n"faf_la_cavale_instru");
            // "wire" the dynamic sound event first
            v.QueueEvent(this.dynamic);
            // then decrease volume with F7
            GameInstance.GetCallbackSystem()
                .RegisterCallback(n"Input/Key", this, n"OnPressF7")
                .AddTarget(InputTarget.Key(EInputKey.IK_F7));
            // or increase volume with F8
            GameInstance.GetCallbackSystem()
                .RegisterCallback(n"Input/Key", this, n"OnPressF8")
                .AddTarget(InputTarget.Key(EInputKey.IK_F8));
        }
    }
    public final func OnPlayerDetach(player: ref<PlayerPuppet>) -> Void {}
    private cb func OnPressF7(evt: ref<KeyInputEvent>) {
        if NotEquals(evt.GetAction(), EInputAction.IACT_Release) { return; }
        if IsDefined(this.dynamic) {
            this.volume = MaxF(0., this.volume - 0.2);
            this.dynamic.SetVolume(this.volume, null);
        }
    }
    private cb func OnPressF8(evt: ref<KeyInputEvent>) {
        if NotEquals(evt.GetAction(), EInputAction.IACT_Release) { return; }
        if IsDefined(this.dynamic) {
            this.volume = MinF(3., this.volume + 0.2);
            this.dynamic.SetVolume(this.volume, null);
        }
    }
}