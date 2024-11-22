module Audioware

import Codeware.Localization.*

class LocalizationProvider extends ModLocalizationProvider {
    protected func OnAttach() {
        this.OnLocaleChange();
        this.OnGenderChange();
    }
    protected func OnLocaleChange() {
        let system = LocalizationSystem.GetInstance(this.GetGameInstance());
        let spoken = system.GetVoiceLanguage();
        let written = system.GetSubtitleLanguage();
        SetGameLocales(spoken, written);
    }
    protected func OnGenderChange() {
        let system = LocalizationSystem.GetInstance(this.GetGameInstance());
        let gender = system.GetPlayerGender();
        SetPlayerGender(gender);
    }
}