import Audioware.AudioSettingsExt
import Audioware.EmitterSettings
import Audioware.EmitterDistances
import Audioware.Tween
import Audioware.LocaleExt
import Codeware.Localization.PlayerGender

@addMethod(GameInstance)
public static func GetAudioSystemExt(game: GameInstance) -> ref<AudioSystemExt> = new AudioSystemExt();

public native class AudioSystemExt {
    // enhanced SDK
    public final native func Play(eventName: CName, entityID: EntityID, emitterName: CName, line: scnDialogLineType, ext: ref<AudioSettingsExt>) -> Void;
    public final native func Stop(eventName: CName, entityID: EntityID, emitterName: CName, tween: ref<Tween>) -> Void;
    public final native func Switch(switchName: CName, switchValue: CName, entityID: EntityID, emitterName: CName, switchNameTween: ref<Tween>, switchValueSettings: ref<AudioSettingsExt>) -> Void;
    public final native func PlayOverThePhone(eventName: CName, emitterName: CName, gender: CName) -> Void;

    // spatial scene
    public final native func RegisterEmitter(entityID: EntityID, tagName: CName, opt emitterName: CName, opt emitterSettings: ref<EmitterSettings>) -> Bool;
    public final native func UnregisterEmitter(entityID: EntityID, tagName: CName) -> Bool;
    public final native func IsRegisteredEmitter(entityID: EntityID, opt tagName: CName) -> Bool;
    public final native func EmittersCount() -> Int32;
    public final native func PlayOnEmitter(eventName: CName, entityID: EntityID, tagName: CName, ext: ref<AudioSettingsExt>) -> Void;
    public final native func StopOnEmitter(eventName: CName, entityID: EntityID, tagName: CName, tween: ref<Tween>) -> Void;
    public final native func OnEmitterDies(entityID: EntityID) -> Void;
    public final native func OnEmitterIncapacitated(entityID: EntityID) -> Void;
    public final native func OnEmitterDefeated(entityID: EntityID) -> Void;
    public final func IsValidEmitter(className: CName) -> Bool = NotEquals(className, n"PlayerPuppet") && Reflection.GetClass(className).IsA(n"gameObject");
    
    // misc
    /// debug or release build ?
    public final native func IsDebug() -> Bool;
    /// returns sound duration as seconds if found, or -1.0 otherwise
    ///
    /// `total` can be used to retrieve total audio duration (default: current audio region duration).
    public final native func Duration(eventName: CName, opt locale: LocaleExt, opt gender: PlayerGender, opt total: Bool) -> Float;
    /// major, minor, patch, type (0 = alpha, 1 = beta, 2 = rc, 3 = official), build number
    public final native func SemanticVersion() -> [Uint16; 5];
    public final func Version() -> String {
        let v = this.SemanticVersion();
        let major = v[0];
        let minor = v[1];
        let patch = v[2];
        let type: String;
        switch v[3] {
            case Cast<Uint16>(0):
                type = "alpha";
                break;
            case Cast<Uint16>(1):
                type = "beta";
                break;
            case Cast<Uint16>(2):
                type = "rc";
                break;
            default:
                type = "official";
                break;
        }
        let build = v[4];
        let triple = s"\(ToString(major)).\(ToString(minor)).\(ToString(patch))";
        // e.g. "1.0.0"
        if Equals(type, "official") { return triple; }
        // e.g. "1.0.0-rc"
        if Equals(build, Cast<Uint16>(0)) { return s"\(triple)-\(type)"; }
        // e.g. "1.0.0-rc.3"
        return s"\(triple)-\(type).\(ToString(build))";
    }
    
    // SDK overloading
    public final func Play(eventName: CName) -> Void {
        let entityID: EntityID;
        let emitterName: CName;
        let line: scnDialogLineType;
        let ext: ref<AudioSettingsExt>;
        this.Play(eventName, entityID, emitterName, line, ext);
    }
    public final func Play(eventName: CName, entityID: EntityID, emitterName: CName) -> Void {
        let line: scnDialogLineType;
        let ext: ref<AudioSettingsExt>;
        this.Play(eventName, entityID, emitterName, line, ext);
    }
    public final func Play(eventName: CName, entityID: EntityID, emitterName: CName, line: scnDialogLineType) -> Void {
        let ext: ref<AudioSettingsExt>;
        this.Play(eventName, entityID, emitterName, line, ext);
    }
    public final func Play(eventName: CName, entityID: EntityID, emitterName: CName, line: scnDialogLineType, tween: ref<Tween>) -> Void {
        let settings = new AudioSettingsExt();
        settings.fadeIn = tween;
        this.Play(eventName, entityID, emitterName, line, settings);
    }

    public final func Stop(eventName: CName) -> Void {
        let entityID: EntityID;
        let emitterName: CName;
        let tween: ref<Tween>;
        this.Stop(eventName, entityID, emitterName, tween);
    }
    public final func Stop(eventName: CName, entityID: EntityID, emitterName: CName) -> Void {
        let tween: ref<Tween>;
        this.Stop(eventName, entityID, emitterName, tween);
    }

    public final func Switch(switchName: CName, switchValue: CName) -> Void {
        let entityID: EntityID;
        let emitterName: CName;
        let switchNameTween: ref<Tween>;
        let settings: ref<AudioSettingsExt>;
        this.Switch(switchName, switchValue, entityID, emitterName, switchNameTween, settings); 
    }
    public final func Switch(switchName: CName, switchValue: CName, entityID: EntityID, emitterName: CName) -> Void {
        let switchNameTween: ref<Tween>;
        let settings: ref<AudioSettingsExt>;
        this.Switch(switchName, switchValue, entityID, emitterName, switchNameTween, settings); 
    }
    public final func Switch(switchName: CName, switchValue: CName, entityID: EntityID, emitterName: CName, switchNameTween: ref<Tween>) -> Void {
        let settings: ref<AudioSettingsExt>;
        this.Switch(switchName, switchValue, entityID, emitterName, switchNameTween, settings); 
    }
    public final func Switch(switchName: CName, switchValue: CName, entityID: EntityID, emitterName: CName, switchNameTween: ref<Tween>, switchValueTween: ref<Tween>) -> Void {
        let settings = new AudioSettingsExt();
        settings.fadeIn = switchValueTween;
        this.Switch(switchName, switchValue, entityID, emitterName, switchNameTween, settings); 
    }

    public final func PlayOnEmitter(eventName: CName, entityID: EntityID, tagName: CName) -> Void {
        let settings: ref<AudioSettingsExt>;
        this.PlayOnEmitter(eventName, entityID, tagName, settings);
    }
    public final func PlayOnEmitter(eventName: CName, entityID: EntityID, tagName: CName, tween: ref<Tween>) -> Void {
        let settings = new AudioSettingsExt();
        settings.fadeIn = tween;
        this.PlayOnEmitter(eventName, entityID, tagName, settings);
    }

    public final func StopOnEmitter(eventName: CName, entityID: EntityID, tagName: CName) -> Void {
        this.StopOnEmitter(eventName, entityID, tagName);
    }
}
