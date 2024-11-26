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

private func PropagateSubtitle(reaction: CName, entityID: EntityID, emitterName: CName, lineType: scnDialogLineType, duration: Float) -> Void {
    if !IsNameValid(reaction) || !EntityID.IsDefined(entityID) { return; }
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
        SubtitleSubSystem.GetInstance(GetGameInstance()).DelayHideSubtitle(line, duration);
    }
}
