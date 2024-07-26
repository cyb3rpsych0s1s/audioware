module Audioware

enum Preset {
    None = 0,
    Underwater = 1,
    OnThePhone = 2,
}

public class Audioware_SettingsDef extends BlackboardDefinition {
    public let PlayerReverb: BlackboardID_Float;
    public let PlayerPreset: BlackboardID_Int;
    public final const func AutoCreateInSystem() -> Bool {
        return true;
    }
    public final const func Initialize(blackboard: ref<IBlackboard>) -> Void {
        blackboard.SetFloat(this.PlayerReverb, 0.);
        blackboard.SetInt(this.PlayerPreset, EnumInt(Preset.None));
    }
}

@addField(AllBlackboardDefinitions)
public let Audioware_Settings: ref<Audioware_SettingsDef>;
