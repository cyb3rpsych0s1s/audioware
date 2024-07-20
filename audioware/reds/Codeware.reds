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
    protected func OnAttach() {
        super.OnAttach();
        FTLog(AsRef("on attach: LocalizationProvider"));
        this.OnLocaleChange();
        this.OnGenderChange();
    }
    public func OnLocaleChange() -> Void {
        FTLog(AsRef("on locale change: LocalizationProvider"));
        let system = LocalizationSystem.GetInstance(this.GetGameInstance());
        let spoken = system.GetVoiceLanguage();
        let written = system.GetSubtitleLanguage();
        SetGameLocales(spoken, written);
    }
    public func OnGenderChange() -> Void {
        FTLog(AsRef("on gender change: LocalizationProvider"));
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
