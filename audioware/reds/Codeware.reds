module Audioware

import Codeware.Localization.*

native func DefineSubtitles(package: ref<LocalizationPackage>);

public class LocalizationPackage extends ModLocalizationPackage {
    public func VoiceLanguage() -> CName {
        return LocalizationSystem.GetInstance(GetGameInstance()).GetVoiceLanguage();
    }
    public func SubtitleLanguage() -> CName {
        return LocalizationSystem.GetInstance(GetGameInstance()).GetSubtitleLanguage();
    }
    protected func DefineSubtitles() -> Void {
        DefineSubtitles(this);
    }
}

public class LocalizationProvider extends ModLocalizationProvider {
    public func OnLocaleChange() -> Void {
        // TODO
    }
    public func OnGenderChange() -> Void {
        let system = LocalizationSystem.GetInstance(this.GetGameInstance());
        let gender = system.GetPlayerGender();
        SetPlayerGender(gender);
    }
    public func GetPackage(language: CName) -> ref<ModLocalizationPackage> {
        // TODO
        return null;
    }
    public func GetFallback() -> CName { return n""; }
}
