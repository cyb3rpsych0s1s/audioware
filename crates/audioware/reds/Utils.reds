module Audioware

import Codeware.Localization.PlayerGender

public func IntoPlayerGender(gender: CName) -> PlayerGender {
    switch gender {
        case n"Female":
        case n"female":
        case n"fem":
            return PlayerGender.Female;
        case n"Male":
        case n"male":
            return PlayerGender.Male;
        default:
            break;
    }
    return PlayerGender.Default;
}

public func IntoLocaleExt(locale: CName) -> LocaleExt {
    switch locale {
        case n"ht-ht":
        case n"Creole":
			return LocaleExt.Creole;
        case n"jp-jp":
        case n"Japanese":
			return LocaleExt.Japanese;
        case n"ar-ar":
        case n"Arabic":
			return LocaleExt.Arabic;
        case n"ru-ru":
        case n"Russian":
			return LocaleExt.Russian;
        case n"zh-cn":
        case n"SimplifiedChinese":
			return LocaleExt.SimplifiedChinese;
        case n"pt-br":
        case n"BrazilianPortuguese":
			return LocaleExt.BrazilianPortuguese;
        case n"sw-ke":
        case n"sw-tz":
        case n"Swahili":
			return LocaleExt.Swahili;
        case n"fr-fr":
        case n"French":
			return LocaleExt.French;
        case n"pl-pl":
        case n"Polish":
			return LocaleExt.Polish;
        case n"es-es":
        case n"Spanish":
			return LocaleExt.Spanish;
        case n"it-it":
        case n"Italian":
			return LocaleExt.Italian;
        case n"de-de":
        case n"German":
			return LocaleExt.German;
        case n"es-mx":
        case n"LatinAmericanSpanish":
        case n"Mexican":
			return LocaleExt.LatinAmericanSpanish;
        case n"kr-kr":
        case n"Korean":
			return LocaleExt.Korean;
        case n"zh-tw":
        case n"TraditionalChinese":
			return LocaleExt.TraditionalChinese;
        case n"cz-cz":
        case n"Czech":
			return LocaleExt.Czech;
        case n"hu-hu":
        case n"Hungarian":
			return LocaleExt.Hungarian;
        case n"tr-tr":
        case n"Turkish":
			return LocaleExt.Turkish;
        case n"th-th":
        case n"Thai":
			return LocaleExt.Thai;
        default:
            break;
    }
    return LocaleExt.English;
}
