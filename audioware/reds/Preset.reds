module Audioware

enum Preset {
    None = 0,
    Underwater = 1,
    OnThePhone = 2,
}

public class Audioware_SettingsDef extends BlackboardDefinition {
    public let ReverbMix: BlackboardID_Float;
    public let AudioPreset: BlackboardID_Int;
    public final const func AutoCreateInSystem() -> Bool {
        return true;
    }
    public final const func Initialize(blackboard: ref<IBlackboard>) -> Void {
        blackboard.SetFloat(this.ReverbMix, 0.);
        blackboard.SetInt(this.AudioPreset, EnumInt(Preset.None));
    }
}

@addField(AllBlackboardDefinitions)
public let Audioware_Settings: ref<Audioware_SettingsDef>;
