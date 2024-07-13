module Audioware

import Codeware.Localization.*

public native func DefineSubtitles(package: ref<LocalizationPackage>);

public class LocalizationPackage extends ModLocalizationPackage {}
