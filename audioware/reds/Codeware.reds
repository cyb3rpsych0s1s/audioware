module Audioware

import Codeware.Localization.*

native func DefineSubtitles(package: ref<LocalizationPackage>);
public native func SupportedLanguages() -> array<CName>;

public class LocalizationPackage extends ModLocalizationPackage {
    public func VoiceLanguage() -> CName    = LocalizationSystem.GetInstance(GetGameInstance()).GetVoiceLanguage();
    public func SubtitleLanguage() -> CName = LocalizationSystem.GetInstance(GetGameInstance()).GetSubtitleLanguage();
    protected func DefineSubtitles() -> Void {
        DefineSubtitles(this);
    }
}

public class LocalizationProvider extends ModLocalizationProvider {
    public func OnLocaleChange() {
        let system = LocalizationSystem.GetInstance(this.GetGameInstance());
        let spoken = system.GetVoiceLanguage();
        let written = system.GetSubtitleLanguage();
        FTLog(s"update locales: spoken: \(NameToString(spoken)), written: \(NameToString(written))");
        SetGameLocales(spoken, written);
    }
    public func OnGenderChange() {
        let system = LocalizationSystem.GetInstance(this.GetGameInstance());
        let gender = system.GetPlayerGender();
        FTLog(s"update player gender: \(ToString(gender))");
        SetPlayerGender(gender);
    }
    public func GetPackage(language: CName) -> ref<ModLocalizationPackage> {
        return new LocalizationPackage();
    }
    public func GetFallback() -> CName = n"";
}