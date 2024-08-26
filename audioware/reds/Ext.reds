import Audioware.AudiowareSystem
import Audioware.EmitterSettings
import Audioware.Tween
import Audioware.AudioSettingsExt
import Audioware.AudioSettingsExtBuilder
import Audioware.LocaleExt
import Audioware.IntoLocaleExt
import Audioware.IntoPlayerGender
import Codeware.Localization.PlayerGender

@addMethod(GameInstance)
public static func GetAudioSystemExt(game: GameInstance) -> ref<AudioSystemExt> {
    return new AudioSystemExt();
}

public native class AudioSystemExt {
    // enhanced SDK
    public final native func Play(eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt line: scnDialogLineType, opt ext: ref<AudioSettingsExt>) -> Void;
    public final native func Stop(eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt tween: ref<Tween>) -> Void
    public final native func Switch(switchName: CName, switchValue: CName, opt entityID: EntityID, opt emitterName: CName, opt switchNameTween: ref<Tween>, opt switchValueSettings: ref<AudioSettingsExt>) -> Void;
    public final native func PlayOverThePhone(eventName: CName, emitterName: CName, gender: CName) -> Void;
    // enhanced SDK variants
    public final func Play(eventName: CName, opt entityID: EntityID, opt emitterName: CName, opt line: scnDialogLineType, opt tween: ref<Tween>) -> Void {
        this.Play(eventName, entityID, emitterName, line, AudioSettingsExtBuilder.Create().WithFadeInTween(tween).Build());
    }
    public final func Switch(switchName: CName, switchValue: CName, opt entityID: EntityID, opt emitterName: CName, opt switchNameTween: ref<Tween>, opt switchValueTween: ref<Tween>) -> Void {
        this.Switch(switchName, switchValue, entityID, emitterName, switchNameTween, AudioSettingsExtBuilder.Create().WithFadeInTween(switchValueTween).Build()); 
    }
        
    // spatial scene
    public final func RegisterEmitter(entityID: EntityID, opt emitterName: CName, opt emitterSettings: EmitterSettings) -> Void { AudiowareSystem.GetInstance(GetGameInstance()).RegisterEmitter(entityID, emitterName, emitterSettings); }
    public final func UnregisterEmitter(entityID: EntityID) -> Void { AudiowareSystem.GetInstance(GetGameInstance()).UnregisterEmitter(entityID); }
    public final func IsValidEmitter(className: CName) -> Bool { return AudiowareSystem.GetInstance(GetGameInstance()).IsValidEmitter(className); }
    public final native func IsRegisteredEmitter(entityID: EntityID) -> Bool;
    public final native func EmittersCount() -> Int32;
    public final native func PlayOnEmitter(eventName: CName, entityID: EntityID, emitterName: CName, opt tween: ref<Tween>) -> Void;
    public final native func StopOnEmitter(eventName: CName, entityID: EntityID, emitterName: CName, opt tween: ref<Tween>) -> Void;
    public final native func OnEmitterDies(entityID: EntityID) -> Void;

    // misc
    public final native func IsDebug() -> Bool;
    /// returns sound region duration as seconds if found, or -1.0 otherwise
    public final native func Duration(eventName: CName, opt locale: LocaleExt, opt gender: PlayerGender) -> Float;
    public final func Duration(eventName: CName, opt locale: CName, opt gender: CName) -> Float {
        let l: LocaleExt = IntoLocaleExt(locale);
        let g: PlayerGender = IntoPlayerGender(gender);
        return this.Duration(eventName, l, g);
    }
    /// major, minor, patch, type (0 = alpha, 1 = beta, 2 = rc, 3 = official), build number
    public final native func SemanticVersion() -> [Uint16; 5];
    public final func Version() -> String {
        let v = this.SemanticVersion();
        let major = v[0];
        let minor = v[1];
        let patch = v[2];
        let type: String;
        switch v[3] {
            case 0:
                type = "alpha";
                break;
            case 1:
                type = "beta";
                break;
            case 2:
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
}
