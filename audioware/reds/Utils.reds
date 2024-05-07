module Audioware

public static func E(msg: String) -> Void {
    ModLog(n"Audioware", AsRef(msg));
}
public static func F(msg: String, opt context: String) -> Void {
    let error = s"[ERROR]";
    if NotEquals(context, "") { error += s" \(context)"; }
    error += s" \(msg)";
    E(error);
}