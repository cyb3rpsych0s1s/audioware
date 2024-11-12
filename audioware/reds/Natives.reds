module Audioware

private static native func OnGameSessionBeforeStart();
private static native func OnGameSessionStart();
private static native func OnGameSessionReady();
private static native func OnGameSessionPause();
private static native func OnGameSessionResume();
private static native func OnGameSessionBeforeEnd();
private static native func OnGameSessionEnd();

private static native func OnGameSystemAttach();
private static native func OnGameSystemPlayerAttach();
private static native func OnGameSystemPlayerDetach();
private static native func OnGameSystemDetach();

private static native func OnUIMenu(value: Bool);
