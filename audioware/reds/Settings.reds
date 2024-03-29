module Audioware

public class AudiowareSettingsDef extends BlackboardDefinition {
    public let PlayerReverb: BlackboardID_Float;
    public final const func AutoCreateInSystem() -> Bool {
        return true;
    }
    public final const func Initialize(blackboard: ref<IBlackboard>) -> Void {
        blackboard.SetFloat(this.PlayerReverb, 0.);
    }
}

@addField(AllBlackboardDefinitions)
public let AudiowareSettings: ref<AudiowareSettingsDef>;