module Audioware

@wrapMethod(MenuScenario_PreGameSubMenu)
protected cb func OnSwitchToEngagementScreen() -> Bool {
    OnEngagementScreen();
    return wrappedMethod();
}

// @wrapMethod(MenuScenario_PreGameSubMenu)
// protected final func OpenSubMenu(menuName: CName, opt userData: ref<IScriptable>) -> Void {
//     FTLog(s"MenuScenario_PreGameSubMenu.OpenSubMenu: \(NameToString(menuName))");
//     wrappedMethod(menuName, userData);
// }

// @wrapMethod(MenuScenario_BaseMenu)
// protected final func OpenSubMenu(menuName: CName, opt userData: ref<IScriptable>) -> Void {
//     FTLog(s"MenuScenario_BaseMenu.OpenSubMenu: \(NameToString(menuName))");
//     wrappedMethod(menuName, userData);
// }
