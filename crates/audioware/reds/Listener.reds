module Audioware

/// whenever game volume audio settings change
public class VolumeSettingsListener extends ConfigVarListener {
    private let game: GameInstance;

    private let master: Float    = 100.;
    private let sfx: Float       = 100.;
    private let dialogue: Float  = 100.;
    private let music: Float     = 100.;
    private let radio: Float     = 100.;
    private let radioport: Float = 100.;

    public func Initialize(game: GameInstance) {
        this.game = game;

        // update settings for Audioware, which loads way earlier
        let settings = GameInstance.GetSettingsSystem(this.game);
        let setting: Float;
        for name in [
            n"MasterVolume",
            n"SfxVolume",
            n"DialogueVolume",
            n"MusicVolume",
            n"CarRadioVolume",
            n"RadioportVolume"
        ] {
            setting = Cast<Float>((settings.GetGroup(n"/audio/volume").GetVar(name) as ConfigVarInt).GetValue());
            this.UpdateVolume(name, setting);
        }
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
                    let setting: Float = Cast<Float>((settings.GetGroup(groupPath).GetVar(varName) as ConfigVarInt).GetValue());
                    this.UpdateVolume(varName, setting);
                    break;
                default:
                    break;
            }
        }
    }
    private func UpdateVolume(name: CName, value: Float) {
        switch name {
            case n"MasterVolume":
                if NotEquals(value, this.master) {
                    this.master = value;
                    SetVolume(name, value);
                }
                break;
            case n"SfxVolume":
                if NotEquals(value, this.sfx) {
                    this.sfx = value;
                    SetVolume(name, value);
                }
                break;
            case n"DialogueVolume":
                if NotEquals(value, this.dialogue) {
                    this.dialogue = value;
                    SetVolume(name, value);
                }
                break;
            case n"MusicVolume":
                if NotEquals(value, this.music) {
                    this.music = value;
                    SetVolume(name, value);
                }
                break;
            case n"CarRadioVolume":
                if NotEquals(value, this.radio) {
                    this.radio = value;
                    SetVolume(name, value);
                }
                break;
            case n"RadioportVolume":
                if NotEquals(value, this.radioport) {
                    this.radioport = value;
                    SetVolume(name, value);
                }
                break;
            default:
                break;
        }
    }
}