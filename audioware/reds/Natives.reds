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

public static native func RegisterEmitter(entityID: EntityID, opt emitterName: CName, opt emitterSettings: EmitterSettings) -> Bool;
public static native func UnregisterEmitter(entityID: EntityID) -> Bool;
