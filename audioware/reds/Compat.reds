module Audioware

import Codeware.Localization.*

private native func UpdatePlayerLocales(voice: CName, subtitle: CName) -> Void;
private native func UpdatePlayerGender(gender: PlayerGender) -> Void;

public class LocalizationProvider extends ModLocalizationProvider {
    public func OnLocaleChange() -> Void {
        let system = LocalizationSystem.GetInstance(this.GetGameInstance());
        let voice = system.GetVoiceLanguage();
        let subtitle = system.GetSubtitleLanguage();
        UpdatePlayerLocales(voice, subtitle);
    }
    public func OnGenderChange() -> Void {
        let system = LocalizationSystem.GetInstance(this.GetGameInstance());
        let gender = system.GetPlayerGender();
        UpdatePlayerGender(gender);
    }
    public func GetPackage(language: CName) -> ref<ModLocalizationPackage> { return null; }
    public func GetFallback() -> CName { return n""; }
}
