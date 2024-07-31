module Audioware

/// whenever game volume audio settings change
public class VolumeSettingsListener extends ConfigVarListener {
    private let game: GameInstance;

    public func Initialize(game: GameInstance) {
        LOG("initialize VolumeSettingsListener");
        this.game = game;
    }

    public func Start() {
        this.Register(n"/audio/volume");
    }

    protected cb func OnVarModified(groupPath: CName, varName: CName, varType: ConfigVarType, reason: ConfigChangeReason) {
        if Equals(groupPath, n"/audio/volume") && Equals(reason, ConfigChangeReason.Accepted) {
            switch varName {
                case n"MasterVolume":
                case n"SfxVolume":
                case n"DialogueVolume":
                case n"MusicVolume":
                case n"CarRadioVolume":
                case n"RadioportVolume":
                    let settings = GameInstance.GetSettingsSystem(this.game);
                    let setting: Double = Cast<Double>((settings.GetGroup(groupPath).GetVar(varName) as ConfigVarInt).GetValue());
                    LOG(s"value: \(ToString(setting)), groupPath: \(NameToString(groupPath)), varName: \(NameToString(varName)), varType: \(ToString(varType)), reason: \(ToString(reason))");
                    SetVolume(varName, setting);
                    break;
                default:
                    break;
            }
        }
    }
}