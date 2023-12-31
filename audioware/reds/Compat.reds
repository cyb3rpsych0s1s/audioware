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
            package.system = LocalizationSystem.GetInstance(this.GetGameInstance());
            return package;
        }
        return null;
    }
    public func GetFallback() -> CName { return n""; }
}

public class LocalizationPackage extends ModLocalizationPackage {
    private let system: ref<LocalizationSystem>;
    public func VoiceLanguage() -> CName {
        return this.system.GetVoiceLanguage();
    }
    public func SubtitleLanguage() -> CName {
        return this.system.GetSubtitleLanguage();
    }
    protected func DefineSubtitles() -> Void {
        DefineEngineSubtitles(this);
    }
}

private func PropagateSubtitle(reaction: CName, entityID: EntityID, emitterName: CName) -> Void {
  if !IsNameValid(reaction) { return; }
  let game = GetGameInstance();
  let target = GameInstance.FindEntityByID(game, entityID);
  if !IsDefined(target) || !target.IsA(n"gameObject") { return; }
  let localization = LocalizationSystem.GetInstance(game);
  let spoken = localization.GetVoiceLanguage();
  let written = localization.GetSubtitleLanguage();
  // if spoken language is not available, abort
  if !StrBeginsWith(NameToString(spoken), "en-") && !StrBeginsWith(NameToString(spoken), "fr-") { return; }
  // only show subtitles if they are available
  if StrBeginsWith(NameToString(written), "en-") || StrBeginsWith(NameToString(written), "fr-") {
    let board: ref<IBlackboard> = GameInstance.GetBlackboardSystem(game).Get(GetAllBlackboardDefs().UIGameData);
    let key: String = NameToString(reaction);
    let subtitle: String = localization.GetSubtitle(key);
    if StrLen(key) > 0 && NotEquals(key, subtitle) {
      let duration: Float = 3.0;
      let line: scnDialogLineData;
      line.duration = duration;
      line.id = CreateCRUID(StringToUint64(NameToString(reaction)));
      line.isPersistent = false;
      line.speaker = target as GameObject;
      line.speakerName = NameToString(emitterName);
      line.text = subtitle;
      line.type = scnDialogLineType.Regular;
      board.SetVariant(GetAllBlackboardDefs().UIGameData.ShowDialogLine, ToVariant([line]), true);
      let callback: ref<HideSubtitleCallback> = new HideSubtitleCallback();
      callback.line = line;
      Audioware.GetInstance(game).m_subtitleDelayID = GameInstance
      .GetDelaySystem(game)
      .DelayCallback(callback, duration);
    }
  }
}
