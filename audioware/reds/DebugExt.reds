import Audioware.*

public class DummyService extends ScriptableService {
    private cb func OnLoad() {
        GameInstance.GetCallbackSystem().UnregisterCallback(n"Entity/Attached", this);
        GameInstance.GetCallbackSystem().UnregisterCallback(n"Entity/Uninitialize", this);
        GameInstance.GetCallbackSystem().UnregisterCallback(n"Entity/Detach", this);
    }
}

public class AutoEmittersService extends ScriptableService {
    private func OnAttach() {
        GameInstance.GetCallbackSystem().UnregisterCallback(n"Entity/Detach", this);
    }
}

public class AutoEmittersSystem extends ScriptableSystem {
    private func OnAttach() {
        FTLog(s"on attach: AutoEmittersSystem");
        GameInstance.GetCallbackSystem().RegisterCallback(n"Input/Key", this, n"OnKeyInput")
        .AddTarget(InputTarget.Key(EInputKey.IK_F1));
    }
    private cb func OnKeyInput(evt: ref<KeyInputEvent>) {
        if NotEquals(evt.GetAction(), EInputAction.IACT_Release) { return; }
        FTLog(s"on key input: AutoEmittersSystem");
        let sounds = [ 
            n"coco_caline",
            n"god_love_us", 
            n"copacabana",
            n"dok_mai_gab_jeh_gan", 
            n"ton",
            n"dimanche_aux_goudes",
            n"feel_good_inc"
        ];
        let eventName = sounds[RandRange(0, ArraySize(sounds) -1)];
        let tween = new LinearTween();
        tween.startTime = RandRangeF(1.0, 3.0);
        tween.duration = RandRangeF(3.0, 4.5);
        let emitterID: EntityID;
        let emitterCName: CName = n"DummyTest";

        let game = this.GetGameInstance();
        let target = GameInstance.GetTargetingSystem(game).GetLookAtObject(GetPlayer(game));
        if !IsDefined(target) { return; }
        emitterID = target.GetEntityID();
        if !GameInstance.GetAudioSystemExt(game).IsRegisteredEmitter(emitterID) {
            GameInstance.GetAudioSystemExt(game).RegisterEmitter(emitterID);
        }
        FTLog(s"play on emitter: AutoEmittersSystem");
        GameInstance.GetAudioSystemExt(game).PlayOnEmitter(eventName, emitterID, emitterCName);
    }
}