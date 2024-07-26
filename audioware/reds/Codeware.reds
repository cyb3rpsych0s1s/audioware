module Audioware

import Codeware.Localization.*

native func DefineSubtitles(package: ref<LocalizationPackage>);
native func SupportedLanguages() -> array<CName>;

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
        LOG("on attach: LocalizationProvider");
        this.OnLocaleChange();
        this.OnGenderChange();
    }
    public func OnLocaleChange() -> Void {
        LOG("on locale change: LocalizationProvider");
        let system = LocalizationSystem.GetInstance(this.GetGameInstance());
        let spoken = system.GetVoiceLanguage();
        let written = system.GetSubtitleLanguage();
        SetGameLocales(spoken, written);
    }
    public func OnGenderChange() -> Void {
        LOG("on gender change: LocalizationProvider");
        let system = LocalizationSystem.GetInstance(this.GetGameInstance());
        let gender = system.GetPlayerGender();
        SetPlayerGender(gender);
    }
    public func GetPackage(language: CName) -> ref<ModLocalizationPackage> {
        let languages = SupportedLanguages();
        let supported = ArrayContains(languages, language);
        if supported {
            let package = new LocalizationPackage();
            return package;
        }
        return null;
    }
    public func GetFallback() -> CName { return n""; }
}

private func PropagateSubtitle(reaction: CName, entityID: EntityID, emitterName: CName, lineType: scnDialogLineType, duration: Float) -> Void {
    if !IsNameValid(reaction) { return; }
    let target = GameInstance.FindEntityByID(GetGameInstance(), entityID);
    if !IsDefined(target) || !target.IsA(n"gameObject") { return; }
    let key: String = NameToString(reaction);
    let subtitle: String = LocalizationSystem.GetInstance(GetGameInstance()).GetSubtitle(key);
    if StrLen(key) > 0 && NotEquals(key, subtitle) {
        let line: scnDialogLineData;
        line.duration = duration;
        line.id = CreateCRUID(StringToUint64(NameToString(reaction)));
        line.isPersistent = false;
        line.speaker = target as GameObject;
        line.speakerName = NameToString(emitterName);
        line.text = subtitle;
        line.type = lineType;
        let board: ref<IBlackboard> = GameInstance.GetBlackboardSystem(GetGameInstance()).Get(GetAllBlackboardDefs().UIGameData);
        board.SetVariant(GetAllBlackboardDefs().UIGameData.ShowDialogLine, ToVariant([line]), true);
        AudiowareSystem.GetInstance(GetGameInstance()).DelayHideSubtitle(line, duration);
    }
}