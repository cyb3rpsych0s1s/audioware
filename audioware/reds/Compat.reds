module Audioware

import Codeware.Localization.*

private native func UpdateEngineLocale(voice: CName, subtitle: CName, interface: CName) -> Void;
private native func UpdateEngineGender(gender: PlayerGender) -> Void;
private native func DefineEngineSubtitles(self: ref<LocalizationPackage>) -> Void;
private native func SupportedEngineLanguages() -> array<CName>;

public class LocalizationProvider extends ModLocalizationProvider {
    public func OnLocaleChange() -> Void {
        let system = LocalizationSystem.GetInstance(this.GetGameInstance());
        let voice = system.GetVoiceLanguage();
        let subtitle = system.GetSubtitleLanguage();
        let interface = system.GetInterfaceLanguage();
        UpdateEngineLocale(voice, subtitle, interface);
    }
    public func OnGenderChange() -> Void {
        let system = LocalizationSystem.GetInstance(this.GetGameInstance());
        let gender = system.GetPlayerGender();
        UpdateEngineGender(gender);
    }
    public func GetPackage(language: CName) -> ref<ModLocalizationPackage> {
        let languages = SupportedEngineLanguages();
        let supported = ArrayContains(languages, language);
        return supported ? new LocalizationPackage() : null;
    }
    public func GetFallback() -> CName { return n""; }
}

public class LocalizationPackage extends ModLocalizationPackage {
    protected func DefineSubtitles() -> Void {
        DefineEngineSubtitles(this);
    }
}
