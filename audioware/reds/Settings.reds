module Audioware

/// whenever game volume audio settings change
public class VolumeSettingsListener extends ConfigVarListener {
    private let game: GameInstance;

    public func Initialize(game: GameInstance) {
        LOG("initialize VolumeSettingsListener");
        this.game = game;

        // update settings for Audioware, which loads way earlier
        let settings = GameInstance.GetSettingsSystem(this.game);
        let setting: Double;
        for name in [
            n"MasterVolume",
            n"SfxVolume",
            n"DialogueVolume",
            n"MusicVolume",
            n"CarRadioVolume",
            n"RadioportVolume"
        ] {
            setting = Cast<Double>((settings.GetGroup(n"/audio/volume").GetVar(name) as ConfigVarInt).GetValue());
            SetVolume(name, setting);
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

public native struct EmitterDistances {
    let min: Float = 1.0;
    let max: Float = 100.0;
}

public native struct EmitterSettings {
    let distances: EmitterDistances;
    let attenuationFunction: ref<Tween>;
    let enableSpatialization: Bool = true;
    let persistUntilSoundsFinish: Bool = false;
}

public native class AudioRegion {
    public native func SetStart(value: Float);
    public native func SetEnd(value: Float);
}

public native class AudioSettingsExtBuilder {
    public native static func Create() -> ref<AudioSettingsExtBuilder>;
    public final native func SetStartPosition(value: Float);
    public final native func SetLoopRegionStarts(value: Float);
    public final native func SetLoopRegionEnds(value: Float);
    public final native func SetVolume(value: Float);
    public final native func SetFadeInTween(value: ref<Tween>);
    public final native func SetPanning(value: Float);
    public final native func SetPlaybackRate(value: Float);
    public final native func Build() -> ref<AudioSettingsExt>;

    // alternate chained syntax
    public final func WithStartPosition(value: Float) -> ref<AudioSettingsExtBuilder> {
        this.SetStartPosition(value);
        return this;
    }
    public final func WithLoopRegionStarts(value: Float) -> ref<AudioSettingsExtBuilder> {
        this.SetLoopRegionStarts(value);
        return this;
    }
    public final func WithLoopRegionEnds(value: Float) -> ref<AudioSettingsExtBuilder> {
        this.SetLoopRegionEnds(value);
        return this;
    }
    public final func WithVolume(value: Float) -> ref<AudioSettingsExtBuilder> {
        this.SetVolume(value);
        return this;
    }
    public final func WithFadeInTween(value: ref<Tween>) -> ref<AudioSettingsExtBuilder> {
        this.SetFadeInTween(value);
        return this;
    }
    public final func WithPanning(value: Float) -> ref<AudioSettingsExtBuilder> {
        this.SetPanning(value);
        return this;
    }
    public final func WithPlaybackRate(value: Float) -> ref<AudioSettingsExtBuilder> {
        this.SetPlaybackRate(value);
        return this;
    }
}

public native importonly class AudioSettingsExt {}
