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
