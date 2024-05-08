module Audioware

import Codeware.Localization.*

private native func UpdateEngineLocale(voice: CName, subtitle: CName) -> Void;
private native func UpdateEngineGender(gender: PlayerGender) -> Void;
private native func DefineEngineSubtitles(self: ref<LocalizationPackage>) -> Void;
public native func SupportedEngineLanguages() -> array<CName>;
public native func GetReactionDuration(name: CName) -> Float;

public class LocalizationProvider extends ModLocalizationProvider {
    public func OnLocaleChange() -> Void {
        let system = LocalizationSystem.GetInstance(this.GetGameInstance());
        let voice = system.GetVoiceLanguage();
        let subtitle = system.GetSubtitleLanguage();
        UpdateEngineLocale(voice, subtitle);
    }
    public func OnGenderChange() -> Void {
        let system = LocalizationSystem.GetInstance(this.GetGameInstance());
        let gender = system.GetPlayerGender();
        UpdateEngineGender(gender);
    }
    public func GetPackage(language: CName) -> ref<ModLocalizationPackage> {
        let languages = SupportedEngineLanguages();
        let supported = ArrayContains(languages, language);
        if supported {
            let package = new LocalizationPackage();
            return package;
        }
        return null;
    }
    public func GetFallback() -> CName { return n""; }
}

public class LocalizationPackage extends ModLocalizationPackage {
    public func VoiceLanguage() -> CName {
        return LocalizationSystem.GetInstance(GetGameInstance()).GetVoiceLanguage();
    }
    public func SubtitleLanguage() -> CName {
        return LocalizationSystem.GetInstance(GetGameInstance()).GetSubtitleLanguage();
    }
    protected func DefineSubtitles() -> Void {
        DefineEngineSubtitles(this);
    }
}

private func PropagateSubtitle(reaction: CName, entityID: EntityID, emitterName: CName, lineType: scnDialogLineType) -> Void {
  if !IsNameValid(reaction) { return; }
  let target = GameInstance.FindEntityByID(GetGameInstance(), entityID);
  if !IsDefined(target) || !target.IsA(n"gameObject") { return; }
  let board: ref<IBlackboard> = GameInstance.GetBlackboardSystem(GetGameInstance()).Get(GetAllBlackboardDefs().UIGameData);
  let key: String = NameToString(reaction);
  let subtitle: String = LocalizationSystem.GetInstance(GetGameInstance()).GetSubtitle(key);
  if StrLen(key) > 0 && NotEquals(key, subtitle) {
      let duration: Float = GetReactionDuration(reaction);
      let line: scnDialogLineData;
      line.duration = duration;
      line.id = CreateCRUID(StringToUint64(NameToString(reaction)));
      line.isPersistent = false;
      line.speaker = target as GameObject;
      line.speakerName = NameToString(emitterName);
      line.text = subtitle;
      line.type = lineType;
      board.SetVariant(GetAllBlackboardDefs().UIGameData.ShowDialogLine, ToVariant([line]), true);
      let callback: ref<HideSubtitleCallback> = new HideSubtitleCallback();
      callback.line = line;
      Audioware.GetInstance(GetGameInstance()).m_subtitleDelayID = GameInstance
      .GetDelaySystem(GetGameInstance())
      .DelayCallback(callback, duration);
  }
}
