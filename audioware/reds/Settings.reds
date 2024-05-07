module Audioware

enum Preset {
    None = 0,
    Underwater = 1,
    OnThePhone = 2,
}

public class AudiowareSettingsDef extends BlackboardDefinition {
    public let PlayerReverb: BlackboardID_Float;
    public let PlayerPreset: BlackboardID_Int;
    public final const func AutoCreateInSystem() -> Bool {
        return true;
    }
    public final const func Initialize(blackboard: wref<IBlackboard>) -> Void {
        if IsDefined(blackboard) {
            blackboard.SetFloat(this.PlayerReverb, 0.);
            blackboard.SetInt(this.PlayerPreset, EnumInt(Preset.None));
        } else { F("blackboard should not be undefined", "AudiowareSettingsDef.Initialize"); }
    }
}

@addField(AllBlackboardDefinitions)
public let AudiowareSettings: ref<AudiowareSettingsDef>;