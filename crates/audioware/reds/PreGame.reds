module Audioware

@wrapMethod(MenuScenario_PreGameSubMenu)
protected cb func OnSwitchToEngagementScreen() -> Bool {
    OnEngagementScreen();
    return wrappedMethod();
}
