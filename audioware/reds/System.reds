module Audioware

public class AudiowareSystem extends ScriptableSystem {
    private func OnAttach() -> Void {
        FTLog(AsRef("on attach: AudiowareSystem"));
        Yolo();
    }
}