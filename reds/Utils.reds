module Audioware.Utils

public static func E(str: String) -> Void {
  if ShowDebugLogsAudioware() {
    LogChannel(n"DEBUG", s"[Audioware] \(str)");
  };
}

public static func F(str: String) -> Void {
  LogError(s"[ERROR] [Audioware] \(str)");
}

public static func EI(id: TweakDBID, str: String) -> Void {
  E(s"[\(TDBID.ToStringDEBUG(id))] \(str)");
}

public static func FI(id: TweakDBID, str: String) -> Void {
  F(s"[\(TDBID.ToStringDEBUG(id))] \(str)");
}
